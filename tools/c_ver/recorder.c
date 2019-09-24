// Including Headers

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <interface/mmal/mmal.h>
#include <interface/mmal/util/mmal_connection.h>
#include <interface/mmal/util/mmal_default_components.h>
#include <interface/mmal/util/mmal_util.h>
#include <interface/mmal/util/mmal_util_params.h>

// Constants

#define MMAL_CAMERA_PREVIEW_PORT 0
#define MMAL_CAMERA_VIDEO_PORT 1
#define MMAL_CAMERA_CAPTURE_PORT 2

// Structs

typedef struct {
    int width;
    int height;
    int bit_rate;
    int frame_rate;
    int max_seconds;
    FILE* output_file;
    char file_path[128];

    MMAL_COMPONENT_T* camera_component;
    MMAL_COMPONENT_T* encoder_component;
    MMAL_CONNECTION_T* encoder_conn;
    MMAL_POOL_T* encoder_pool;
} VideoState;

// Local Function Declarations

static void camera_callback(MMAL_PORT_T* port, MMAL_BUFFER_HEADER_T* buffer);
static int connect_ports(MMAL_PORT_T* output_port, MMAL_PORT_T* input_port, MMAL_CONNECTION_T** conn);
static int create_camera_component(VideoState* state);
static int create_encoder_component(VideoState* state);
static void destroy_camera_component(VideoState* state);
static void destroy_encoder_component(VideoState* state);
static void disable_port(MMAL_PORT_T* port);
static void encoder_callback(MMAL_PORT_T* port, MMAL_BUFFER_HEADER_T* buffer);
static void set_camera_config(MMAL_COMPONENT_T* camera, VideoState* state);
static void set_default_state(VideoState* state);
static int set_port_format(MMAL_PORT_T* port, VideoState* state);
static void signal_handler(int signal_num);

// Public Function Definitions

int
main()
{
    MMAL_PORT_T* encoder_output_port = NULL;
    VideoState state = {0};
    int failed = 1;

    signal(SIGINT, signal_handler);
    set_default_state(&state);

    do
    {
        MMAL_PORT_T* camera_video_port = NULL;
        MMAL_PORT_T* encoder_input_port = NULL;
        MMAL_STATUS_T status = MMAL_SUCCESS;
        int encoder_queue_len = 0;
        int i =0;

        if (create_camera_component(&state))
        {
            break;
        }

        if (create_encoder_component(&state))
        {
            break;
        }

        camera_video_port = state.camera_component->output[MMAL_CAMERA_VIDEO_PORT];
        encoder_input_port = state.encoder_component->input[0];
        if (connect_ports(camera_video_port, encoder_input_port, &state.encoder_conn))
        {
            break;
        }

        state.output_file = fopen(state.file_path, "wb");
        if (!state.output_file)
        {
            break;
        }

        encoder_output_port = state.encoder_component->output[0];
        encoder_output_port->userdata = (struct MMAL_PORT_USERDATA_T*)&state;
        status = mmal_port_enable(encoder_output_port, encoder_callback);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        camera_video_port = state.camera_component->output[MMAL_CAMERA_VIDEO_PORT];
        status = mmal_port_parameter_set_boolean(camera_video_port, MMAL_PARAMETER_CAPTURE, 1);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        encoder_queue_len = mmal_queue_length(state.encoder_pool->queue);
        for (i = 0; i < encoder_queue_len; ++i)
        {
            MMAL_BUFFER_HEADER_T* buffer = mmal_queue_get(state.encoder_pool->queue);
            assert(buffer);

            status = mmal_port_send_buffer(encoder_output_port, buffer);
            assert(status == MMAL_SUCCESS);
        }

        vcos_sleep(state.max_seconds * 1000);
        failed = 0;
    }
    while (0);

    disable_port(encoder_output_port);
    mmal_connection_destroy(state.encoder_conn);

    if (state.output_file)
    {
        fclose(state.output_file);
    }

    mmal_component_disable(state.encoder_component);
    mmal_component_disable(state.camera_component);

    destroy_encoder_component(&state);
    destroy_camera_component(&state);

    return failed;
}

// Local Function Definitions

void
camera_callback(MMAL_PORT_T* port, MMAL_BUFFER_HEADER_T* buffer)
{
    mmal_buffer_header_release(buffer);
}

int
connect_ports(MMAL_PORT_T* output_port, MMAL_PORT_T* input_port, MMAL_CONNECTION_T** conn)
{
    int failed = 1;

    do
    {
        MMAL_STATUS_T status = MMAL_SUCCESS;

        status = mmal_connection_create(
            conn,
            output_port,
            input_port,
            MMAL_CONNECTION_FLAG_TUNNELLING | MMAL_CONNECTION_FLAG_ALLOCATION_ON_INPUT
        );

        if (status != MMAL_SUCCESS)
        {
            break;
        }

        status = mmal_connection_enable(*conn);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        failed = 0;
    }
    while (0);

    if (failed)
    {
        mmal_connection_destroy(*conn);
        *conn = NULL;
    }

    return failed;
}

int
create_camera_component(VideoState* state)
{
    int failed = 1;

    do
    {
        MMAL_COMPONENT_T* camera = NULL;
        MMAL_ES_FORMAT_T* format = NULL;
        MMAL_PORT_T* capture_port = NULL;
        MMAL_PORT_T* preview_port = NULL;
        MMAL_PORT_T* video_port = NULL;
        MMAL_STATUS_T status = MMAL_SUCCESS;

        status = mmal_component_create(MMAL_COMPONENT_DEFAULT_CAMERA, &camera);
        if (!(status == MMAL_SUCCESS && camera && camera->output_num > 0))
        {

            break;
        }

        capture_port = camera->output[MMAL_CAMERA_CAPTURE_PORT];
        preview_port = camera->output[MMAL_CAMERA_PREVIEW_PORT];
        video_port = camera->output[MMAL_CAMERA_VIDEO_PORT];

        status = mmal_port_enable(camera->control, camera_callback);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        set_camera_config(camera, state);

        failed = set_port_format(capture_port, state) ||
            set_port_format(preview_port, state) ||
            set_port_format(video_port, state);

        if (failed)
        {
            break;
        }

        status = mmal_component_enable(camera);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        state->camera_component = camera;

        failed = 0;
    }
    while (0);

    if (failed)
    {
        destroy_camera_component(state);
    }

    return failed;
}

int
create_encoder_component(VideoState* state)
{
    int failed = 1;

    do
    {
        MMAL_COMPONENT_T *encoder = NULL;
        MMAL_POOL_T *pool = NULL;
        MMAL_PORT_T* encoder_input = NULL;
        MMAL_PORT_T* encoder_output = NULL;
        MMAL_STATUS_T status = MMAL_SUCCESS;

        status = mmal_component_create(MMAL_COMPONENT_DEFAULT_VIDEO_ENCODER, &encoder);
        if (!(status == MMAL_SUCCESS && encoder && encoder->output_num > 0))
        {
            break;
        }

        encoder_input = encoder->input[0];
        encoder_output = encoder->output[0];
        mmal_format_copy(encoder_output->format, encoder_input->format);

        encoder_output->format->encoding = MMAL_ENCODING_H264;
        encoder_output->format->bitrate = state->bit_rate;

        encoder_output->buffer_size = encoder_output->buffer_size_recommended;
        if (encoder_output->buffer_size < encoder_output->buffer_size_min)
        {
            encoder_output->buffer_size = encoder_output->buffer_size_min;
        }

        encoder_output->buffer_num = encoder_output->buffer_num_recommended;
        if (encoder_output->buffer_num < encoder_output->buffer_num_min)
        {
            encoder_output->buffer_num = encoder_output->buffer_num_min;
        }

        status = mmal_port_format_commit(encoder_output);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        status = mmal_component_enable(encoder);
        if (status != MMAL_SUCCESS)
        {
            break;
        }

        pool = mmal_port_pool_create(
            encoder_output,
            encoder_output->buffer_num,
            encoder_output->buffer_size
        );

        if (!pool)
        {
            break;
        }

        state->encoder_pool = pool;
        state->encoder_component = encoder;

        failed = 0;
    }
    while (0);

    if (failed)
    {
        destroy_encoder_component(state);
    }

    return failed;
}

void
destroy_camera_component(VideoState* state)
{
   if (state->camera_component)
   {
      mmal_component_destroy(state->camera_component);
      state->camera_component = NULL;
   }
}


void
destroy_encoder_component(VideoState* state)
{
    if (state->encoder_pool)
    {
        mmal_port_pool_destroy(state->encoder_component->output[0], state->encoder_pool);
        state->encoder_pool = NULL;
    }

    if (state->encoder_component)
    {
        mmal_component_destroy(state->encoder_component);
        state->encoder_component = NULL;
    }
}

void
disable_port(MMAL_PORT_T* port)
{
    if (port && port->is_enabled)
    {
        mmal_port_disable(port);
    }
}

void
encoder_callback(MMAL_PORT_T* port, MMAL_BUFFER_HEADER_T* buffer)
{
    VideoState* state = (VideoState*) port->userdata;

    assert(state);
    assert(state->output_file);

    if (buffer->length)
    {
        mmal_buffer_header_mem_lock(buffer);
        fwrite(buffer->data, 1, buffer->length, state->output_file);
        mmal_buffer_header_mem_unlock(buffer);
    }

    mmal_buffer_header_release(buffer);

    if (port->is_enabled)
    {
      MMAL_BUFFER_HEADER_T* new_buffer = NULL;
      MMAL_STATUS_T status = MMAL_SUCCESS;

      new_buffer = mmal_queue_get(state->encoder_pool->queue);
      assert(new_buffer);

      status = mmal_port_send_buffer(port, new_buffer);
      assert(status == MMAL_SUCCESS);
   }
}

void
set_camera_config(MMAL_COMPONENT_T* camera, VideoState* state)
{
    if (!(camera && state))
    {
        assert(0);
        return;
    }

    MMAL_PARAMETER_CAMERA_CONFIG_T cam_config = {
        { MMAL_PARAMETER_CAMERA_CONFIG, sizeof(cam_config) },
        .max_stills_w = state->width,
        .max_stills_h = state->height,
        .stills_yuv422 = 0,
        .one_shot_stills = 0,
        .max_preview_video_w = state->width,
        .max_preview_video_h = state->height,
        .num_preview_video_frames = 3,
        .stills_capture_circular_buffer_height = 0,
        .fast_preview_resume = 0,
        .use_stc_timestamp = MMAL_PARAM_TIMESTAMP_MODE_RESET_STC
    };

    mmal_port_parameter_set(camera->control, &cam_config.hdr);
}

void
set_default_state(VideoState* state)
{
    size_t file_path_len = sizeof(state->file_path);

    if (!state)
    {
        assert(0);
        return;
    }

   memset(state, 0, sizeof(VideoState));

   state->width = 1920;
   state->height = 1080;
   state->bit_rate = 17000000;
   state->frame_rate = 30;
   state->max_seconds = 5;

   memset(state->file_path, 0, file_path_len);
   snprintf(state->file_path, file_path_len, "%d.h264", (int) time(NULL));
}

int
set_port_format(MMAL_PORT_T* port, VideoState* state)
{
    MMAL_ES_FORMAT_T* format = NULL;
    MMAL_STATUS_T status = MMAL_SUCCESS;

    if (!(port && state))
    {
        assert(0);
        return 1;
    }

    format = port->format;
    if (!format)
    {
        assert(0);
        return 1;
    }

    format->encoding = MMAL_ENCODING_OPAQUE;
    format->encoding_variant = MMAL_ENCODING_I420;

    format->es->video.width = state->width;
    format->es->video.height = state->height;
    format->es->video.crop.x = 0;
    format->es->video.crop.y = 0;
    format->es->video.crop.width = state->width;
    format->es->video.crop.height = state->height;
    format->es->video.frame_rate.num = state->frame_rate;
    format->es->video.frame_rate.den = 1;

    status = mmal_port_format_commit(port);
    if (status != MMAL_SUCCESS)
    {
        return 1;
    }

    if (port->buffer_num < 3)
    {
        port->buffer_num = 3;
    }

    return 0;
}

void
signal_handler(int signal_num)
{
   exit(1);
}
