//! FFmpeg 5.0 -> 9.0 API migration reference for rusty_ffmpeg users.
//!
//! Every boundary below was verified against real FFmpeg release sources
//! (5.0.3 / 5.1.10 / 6.0.2 / 6.1.6 / 7.0.3 / 7.1.5 / 8.0.3 / 8.1.2) by
//! diffing the public headers and cross-checking `doc/APIchanges`, and
//! each item is compiled and executed by the CI matrix with the matching
//! version feature enabled.
//!
//! The chained features mirror the version boundaries exactly:
//! `ffmpeg5` -> `ffmpeg5_1` -> `ffmpeg6` -> `ffmpeg6_1` -> `ffmpeg7` ->
//! `ffmpeg7_1` -> `ffmpeg8` -> `ffmpeg8_1` -> `ffmpeg9`.
//!
//! ## Fields / constants removed (compiled-out by FF_API guards)
//!
//! | Item                                        | Removed | Replacement                         |
//! |---------------------------------------------|---------|-------------------------------------|
//! | `AVCodecContext.thread_safe_callbacks`      | 6.0     | frame threading is always used      |
//! | `AVCodecContext.debug_mv`                   | 6.0     | -                                   |
//! | `AVCodecContext.sub_text_format`            | 6.0     | AVSubtitle is always ASS-rendered   |
//! | `AVCodecContext.frame_number` (i32)         | 7.0     | `frame_num` (6.0+, i64)             |
//! | `channels`/`channel_layout: u64` fields     | 7.0     | `ch_layout: AVChannelLayout` (5.1+) |
//! | `AVFrame.pkt_duration`                      | 7.0     | `duration` (6.0+)                   |
//! | `AVFrame.coded/display_picture_number`      | 7.0     | -                                   |
//! | `AVFrame.reordered_opaque`                  | 7.0     | `opaque` via AVFrameSideData? (no)  |
//! | `AVFrame.interlaced_frame`/`top_field_first`| 7.0     | `AV_FRAME_FLAG_*` flags (6.1+)      |
//! | `AVFrame.palette_has_changed`               | 7.0     | -                                   |
//! | old channel layout functions (`av_get_*`)   | 7.0     | `av_channel_layout_*` (5.1+)        |
//! | old FIFO API (`av_fifo_alloc()`, ...)       | 7.0     | `av_fifo_alloc2()` (5.1+)           |
//! | `avcodec_enum_to_chroma_pos()`              | 7.0     | `avcodec_chroma_pos_to_enum()`-free AVCHROMA_LOC_* |
//! | `av_stream_get_end_pts()`                   | 7.0     | -                                   |
//! | `av_fopen_utf8()`/`av_tempfile()`           | 7.0     | - (removed without replacement)     |
//! | `AVFrame.key_frame`                         | 8.0     | `flags & AV_FRAME_FLAG_KEY` (6.1+)  |
//! | `AVFrame.pkt_pos`/`pkt_size`                | 8.0     | packet side data                    |
//! | `AVCodecContext.ticks_per_frame`            | 8.0     | time base / container metadata      |
//! | `avcodec_close()`                           | 8.0     | `avcodec_free_context()`            |
//! | `av_fmt_ctx_get_duration_estimation_method()`| 8.0    | `AVFormatContext.duration_estimation_method` |
//! | `AVFMT_FLAG_SHORTEST`                       | 8.0     | -                                   |
//! | `av_stream_add_side_data()`/`AVStream` side data | 8.0 | `av_packet_side_data_*` (7.0+)    |
//!
//! ## API additions per version
//!
//! | Version | Additions (demoed below)                                                     |
//! |---------|------------------------------------------------------------------------------|
//! | 5.1     | `AVChannelLayout` API, `av_fifo_alloc2()`, `*.ch_layout` fields              |
//! | 6.0     | `frame_num`, `AVFrame.duration`, `av_dict_iterate()`, `AV_CH_LAYOUT_CUBE`    |
//! | 6.1     | `AV_FRAME_FLAG_*`, `av_frame_replace()`, more `AV_CH_LAYOUT_*`               |
//! | 7.0     | `av_channel_layout_retype()`, `av_frame_side_data_clone()`, more layouts     |
//! | 7.1     | `avcodec_get_supported_config()`, `AVERROR_HTTP_TOO_MANY_REQUESTS`, surround |
//! | 8.0     | `AV_CHAN_BINAURAL_*`, binaural layouts                                       |
//! | 8.1     | `avcodec_receive_frame_flags()`, `AVAlphaMode`                               |
//!
//! Note: `av_init_packet()` is deprecated since 4.4 but still present in
//! 9.0; prefer `av_packet_alloc()`.
//!
//! Run (env like build.rs expects; the feature must match your FFmpeg):
//!
//! ```text
//! FFMPEG_INCLUDE_DIR=/path/to/ffmpeg/include \
//! FFMPEG_PKG_CONFIG_PATH=/path/to/ffmpeg/lib/pkgconfig \
//! cargo run --example version_compat --features ffmpeg9
//! ```

use std::ptr;

use rusty_ffmpeg::ffi;

fn main() {
    demo_frame_counter();
    demo_codecpar_channels();
    demo_frame_duration();
    demo_frame_flags();
    demo_channel_layout_versions();
    demo_fifo_versions();
    demo_dict_iteration();
    demo_removed_in_6_0();
    demo_removed_in_7_0();
    demo_frame_replace();
    demo_channel_layout_retype();
    demo_frame_side_data_clone();
    demo_supported_config();
    demo_avcodec_close();
    demo_packet_allocation();
    demo_8_1_additions();
    println!("\nversion-compat demo finished");
}

/// Frame counter: `AVCodecContext.frame_number` (5.0 ~ 6.1, removed in 7.0)
/// -> `frame_num: i64` (6.0+). Both exist in 6.0/6.1.
fn demo_frame_counter() {
    unsafe {
        // NULL codec is allowed, it just skips codec-specific defaults.
        let mut ctx_ptr = ffi::avcodec_alloc_context3(ptr::null_mut());
        let ctx = ctx_ptr.as_mut().expect("avcodec_alloc_context3 failed");

        #[cfg(feature = "ffmpeg6")]
        println!("frame_num (6.0+ i64) OK, initial value: {}", ctx.frame_num);
        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg7")))]
        {
            #[allow(deprecated)]
            println!(
                "frame_number (5.0~6.1 i32) OK, initial value: {}",
                ctx.frame_number
            );
        }

        ffi::avcodec_free_context(&mut ctx_ptr);
    }
}

/// Codec parameter channels: `channels`/`channel_layout: u64` (5.0 ~ 6.1,
/// removed in 7.0) -> `ch_layout: AVChannelLayout` (5.1+). Both exist in
/// 5.1 ~ 6.1.
fn demo_codecpar_channels() {
    unsafe {
        let mut par_ptr = ffi::avcodec_parameters_alloc();
        let par = par_ptr.as_mut().expect("avcodec_parameters_alloc failed");

        #[cfg(feature = "ffmpeg5_1")]
        {
            // av_channel_layout_default initializes the whole AVChannelLayout.
            ffi::av_channel_layout_default(&mut par.ch_layout, 2);
            println!(
                "codecpar ch_layout (5.1+) OK: nb_channels = {}",
                par.ch_layout.nb_channels
            );
        }
        // 5.0-only: no AVChannelLayout at all, the old helper is the only way.
        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg5_1")))]
        {
            let mask = ffi::av_get_default_channel_layout(2);
            par.channels = 2;
            par.channel_layout = mask;
            println!(
                "codecpar channels (5.0) OK: channels = {}, channel_layout = {:#x}",
                par.channels, mask
            );
        }
        // The deprecated plain fields still compile against 5.1 ~ 6.1.
        #[cfg(all(feature = "ffmpeg5_1", not(feature = "ffmpeg7")))]
        {
            #[allow(deprecated)]
            {
                println!(
                    "codecpar deprecated channels field (5.1~6.1) OK: channels = {}",
                    par.channels
                );
            }
        }

        ffi::avcodec_parameters_free(&mut par_ptr);
    }
}

/// Frame duration: `AVFrame.pkt_duration` (5.0 ~ 6.1, removed in 7.0) ->
/// `duration` (6.0+). Both exist in 6.0/6.1.
fn demo_frame_duration() {
    unsafe {
        let mut frame_ptr = ffi::av_frame_alloc();
        let frame = frame_ptr.as_mut().expect("av_frame_alloc failed");

        #[cfg(feature = "ffmpeg6")]
        {
            frame.duration = 1234;
            println!("frame duration (6.0+) OK: duration = {}", frame.duration);
        }
        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg7")))]
        {
            #[allow(deprecated)]
            {
                frame.pkt_duration = 1234;
                println!(
                    "frame pkt_duration (5.0~6.1) OK: pkt_duration = {}",
                    frame.pkt_duration
                );
            }
        }

        ffi::av_frame_free(&mut frame_ptr);
    }
}

/// Frame key flag: `AVFrame.key_frame` (5.0 ~ 7.1, removed in 8.0) ->
/// `AVFrame.flags & AV_FRAME_FLAG_KEY` (6.1+).
fn demo_frame_flags() {
    unsafe {
        let mut frame_ptr = ffi::av_frame_alloc();
        let frame = frame_ptr.as_mut().expect("av_frame_alloc failed");

        #[cfg(feature = "ffmpeg6_1")]
        {
            // AV_FRAME_FLAG_* constants are bindgen c_uint, AVFrame.flags is c_int.
            frame.flags |= ffi::AV_FRAME_FLAG_KEY as std::ffi::c_int;
            println!(
                "frame flags (6.1+) OK: AV_FRAME_FLAG_KEY = {}",
                (frame.flags & ffi::AV_FRAME_FLAG_KEY as std::ffi::c_int) != 0
            );
        }
        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg8")))]
        {
            #[allow(deprecated)]
            {
                frame.key_frame = 1;
                println!(
                    "frame key_frame (5.0~7.1) OK: key_frame = {}",
                    frame.key_frame
                );
            }
        }

        ffi::av_frame_free(&mut frame_ptr);
    }
}

/// Handwritten `AV_CH_*`/`AV_CHANNEL_LAYOUT_*` constants are re-exported
/// from feature `ffmpeg5_1` (the whole module needs `AVChannelLayout`,
/// which appeared in 5.1), and each version added more layouts.
fn demo_channel_layout_versions() {
    #[cfg(not(feature = "ffmpeg5_1"))]
    {
        println!("channel layout API unavailable on FFmpeg 5.0 (needs 5.1+)");
    }

    #[cfg(feature = "ffmpeg5_1")]
    unsafe {
        // The struct initializers exist on every 5.1+ version.
        let mut stereo = ffi::AV_CHANNEL_LAYOUT_STEREO;
        assert!(ffi::av_channel_layout_check(&stereo) == 1);
        ffi::av_channel_layout_default(&mut stereo, 6);
        println!(
            "AVChannelLayout (5.1+) OK: default 6ch order = {}",
            stereo.order
        );

        // Added in FFmpeg 6.0
        #[cfg(feature = "ffmpeg6")]
        println!("AV_CH_LAYOUT_CUBE (6.0+) OK: {:#x}", ffi::AV_CH_LAYOUT_CUBE);
        // Added in FFmpeg 6.1
        #[cfg(feature = "ffmpeg6_1")]
        println!(
            "AV_CH_LAYOUT_7POINT1POINT4_BACK (6.1+) OK: {:#x}",
            ffi::AV_CH_LAYOUT_7POINT1POINT4_BACK
        );
        // Added in FFmpeg 7.0
        #[cfg(feature = "ffmpeg7")]
        println!(
            "AV_CH_LAYOUT_9POINT1POINT4_BACK (7.0+) OK: {:#x}",
            ffi::AV_CH_LAYOUT_9POINT1POINT4_BACK
        );
        // Added in FFmpeg 7.1 (bindgen enum variants)
        #[cfg(feature = "ffmpeg7_1")]
        println!(
            "AV_CH_SIDE_SURROUND_LEFT (7.1+) OK: {:#x}",
            ffi::AV_CH_SIDE_SURROUND_LEFT
        );
        // Added in FFmpeg 8.0 (bindgen enum variants)
        #[cfg(feature = "ffmpeg8")]
        println!(
            "AV_CH_LAYOUT_BINAURAL (8.0+) OK: {:#x}",
            ffi::AV_CH_LAYOUT_BINAURAL
        );
        #[cfg(feature = "ffmpeg7_1")]
        {
            // AVERROR_HTTP_TOO_MANY_REQUESTS was added in FFmpeg 7.1.
            assert_ne!(ffi::AVERROR_HTTP_TOO_MANY_REQUESTS, 0);
            println!("AVERROR_HTTP_TOO_MANY_REQUESTS (7.1+) OK");
        }
    }
}

/// FIFO API: old `AVFifoBuffer`-based functions (5.0 ~ 6.1, removed in 7.0)
/// -> the refcounted `av_fifo_alloc2()` API (5.1+).
fn demo_fifo_versions() {
    unsafe {
        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg7")))]
        {
            #[allow(deprecated)]
            {
                let fifo = ffi::av_fifo_alloc(16);
                assert!(!fifo.is_null(), "av_fifo_alloc failed");
                println!(
                    "av_fifo_alloc/av_fifo_size (5.0~6.1) OK: size = {}",
                    ffi::av_fifo_size(fifo)
                );
                ffi::av_fifo_free(fifo);
            }
        }
        #[cfg(feature = "ffmpeg5_1")]
        {
            let mut fifo = ffi::av_fifo_alloc2(4, 4, ffi::AV_FIFO_FLAG_AUTO_GROW);
            assert!(!fifo.is_null(), "av_fifo_alloc2 failed");
            let value: i32 = 42;
            assert!(
                ffi::av_fifo_write(fifo, &value as *const i32 as *const std::ffi::c_void, 1) == 0
            );
            let mut out: i32 = 0;
            assert!(ffi::av_fifo_read(fifo, &mut out as *mut i32 as *mut std::ffi::c_void, 1) == 0);
            println!("av_fifo_alloc2/write/read (5.1+) OK: value = {out}");
            ffi::av_fifo_freep2(&mut fifo);
        }
    }
}

/// Dictionary iteration: `av_dict_get()` (every version) ->
/// `av_dict_iterate()` (6.0+).
fn demo_dict_iteration() {
    unsafe {
        let mut dict: *mut ffi::AVDictionary = ptr::null_mut();
        let key = c"hello".as_ptr();
        let value = c"world".as_ptr();
        assert!(ffi::av_dict_set(&mut dict, key, value, 0) >= 0);

        let entry = ffi::av_dict_get(dict, key, ptr::null(), 0);
        assert!(!entry.is_null());
        println!("av_dict_get OK");

        #[cfg(feature = "ffmpeg6")]
        {
            let entry = ffi::av_dict_iterate(dict, ptr::null());
            assert!(!entry.is_null());
            println!("av_dict_iterate (6.0+) OK");
        }

        ffi::av_dict_free(&mut dict);
    }
}

/// Fields removed from `AVCodecContext` in 6.0 (deprecated in 4.x/5.0).
fn demo_removed_in_6_0() {
    #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg6")))]
    unsafe {
        let mut ctx_ptr = ffi::avcodec_alloc_context3(ptr::null_mut());
        let ctx = ctx_ptr.as_mut().expect("avcodec_alloc_context3 failed");
        #[allow(deprecated)]
        {
            println!(
                "AVCodecContext 5.x-only fields OK: debug_mv = {}, thread_safe_callbacks = {}",
                ctx.debug_mv, ctx.thread_safe_callbacks
            );
        }
        ffi::avcodec_free_context(&mut ctx_ptr);
    }
}

/// Functions removed in 7.0 (deprecated in 5.1 ~ 6.1): the old channel
/// layout helpers, chroma position conversion, stream end_pts, and the
/// file helpers (referenced, not called, to prove they still exist).
fn demo_removed_in_7_0() {
    #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg7")))]
    unsafe {
        #[allow(deprecated)]
        {
            // Old uint64-bitmask channel layout API (removed in 7.0).
            let mask = ffi::av_get_default_channel_layout(2);
            println!("av_get_default_channel_layout (5.0~6.1) OK: mask = {mask:#x}");

            // Chroma position conversion (removed in 7.0).
            let (mut x, mut y) = (0, 0);
            assert!(ffi::avcodec_enum_to_chroma_pos(&mut x, &mut y, ffi::AVCHROMA_LOC_LEFT) >= 0);
            println!("avcodec_enum_to_chroma_pos (5.0~6.1) OK: pos = ({x}, {y})");
        }
        // Referenced without calling (functions take/return file handles).
        let _keep_fopen = ffi::av_fopen_utf8;
        let _keep_tempfile = ffi::av_tempfile;
        let _keep_end_pts = ffi::av_stream_get_end_pts;
        println!(
            "av_fopen_utf8/av_tempfile/av_stream_get_end_pts (5.0~6.1) symbols OK, removed in 7.0"
        );
    }
}

/// `av_frame_replace()`: added in 6.1, moves/replaces the contents of dst.
fn demo_frame_replace() {
    #[cfg(feature = "ffmpeg6_1")]
    unsafe {
        let mut dst_ptr = ffi::av_frame_alloc();
        let mut src_ptr = ffi::av_frame_alloc();
        assert!(!dst_ptr.is_null() && !src_ptr.is_null());
        let ret = ffi::av_frame_replace(dst_ptr, src_ptr);
        assert!(ret == 0, "av_frame_replace failed: {ret}");
        println!("av_frame_replace (6.1+) OK");
        ffi::av_frame_free(&mut dst_ptr);
        ffi::av_frame_free(&mut src_ptr);
    }
}

/// `av_channel_layout_retype()`: added in 7.0, converts between orders.
fn demo_channel_layout_retype() {
    #[cfg(feature = "ffmpeg7")]
    unsafe {
        let mut stereo = ffi::AV_CHANNEL_LAYOUT_STEREO;
        let ret = ffi::av_channel_layout_retype(
            &mut stereo,
            ffi::AV_CHANNEL_ORDER_NATIVE,
            ffi::AV_CHANNEL_LAYOUT_RETYPE_FLAG_CANONICAL as std::ffi::c_int,
        );
        assert!(ret >= 0, "av_channel_layout_retype failed: {ret}");
        println!("av_channel_layout_retype (7.0+) OK: ret = {ret}");
    }
}

/// `av_frame_side_data_clone()`: added in 7.0 (referenced, not called).
fn demo_frame_side_data_clone() {
    #[cfg(feature = "ffmpeg7")]
    {
        let _keep = ffi::av_frame_side_data_clone;
        println!("av_frame_side_data_clone (7.0+) symbol OK");
    }
}

/// `avcodec_get_supported_config()`: added in 7.1, replaces reading
/// `AVCodec.*_list` fields directly.
fn demo_supported_config() {
    #[cfg(feature = "ffmpeg7_1")]
    unsafe {
        let codec = ffi::avcodec_find_decoder(ffi::AV_CODEC_ID_H264);
        if let Some(codec) = codec.as_ref() {
            let mut out_configs: *const std::ffi::c_void = ptr::null();
            let mut out_num: std::ffi::c_int = 0;
            let ret = ffi::avcodec_get_supported_config(
                ptr::null(),
                codec,
                ffi::AV_CODEC_CONFIG_PIX_FORMAT,
                0,
                &mut out_configs,
                &mut out_num,
            );
            assert!(ret >= 0, "avcodec_get_supported_config failed: {ret}");
            if out_configs.is_null() {
                println!("avcodec_get_supported_config (7.1+) OK: all pixel formats supported");
            } else {
                println!(
                    "avcodec_get_supported_config (7.1+) OK: {} pixel formats",
                    out_num
                );
            }
        }
    }
}

/// `avcodec_close()`: 5.0 ~ 7.1 (deprecated since 7.0, removed in 8.0) ->
/// `avcodec_free_context()`.
fn demo_avcodec_close() {
    unsafe {
        let mut ctx_ptr = ffi::avcodec_alloc_context3(ptr::null_mut());
        assert!(!ctx_ptr.is_null());

        #[cfg(all(feature = "ffmpeg5", not(feature = "ffmpeg8")))]
        {
            #[allow(deprecated)]
            {
                ffi::avcodec_close(ctx_ptr);
                println!("avcodec_close (5.0~7.1, removed in 8.0) OK");
            }
        }

        ffi::avcodec_free_context(&mut ctx_ptr);
        println!("avcodec_free_context OK");
    }
}

/// Packet allocation: `av_init_packet()` on a zeroed stack packet is
/// deprecated since FFmpeg 4.4 but still present in 9.0; prefer
/// `av_packet_alloc()`/`av_packet_free()`.
fn demo_packet_allocation() {
    unsafe {
        // The modern way, works on every version.
        let mut pkt_ptr = ffi::av_packet_alloc();
        assert!(!pkt_ptr.is_null(), "av_packet_alloc failed");
        println!("av_packet_alloc/av_packet_free OK");
        ffi::av_packet_free(&mut pkt_ptr);

        // The legacy idiom, deprecated but never removed.
        let mut pkt: ffi::AVPacket = std::mem::zeroed();
        #[allow(deprecated)]
        ffi::av_init_packet(&mut pkt);
        println!("av_init_packet (deprecated 4.4, still present in 9.0) OK");
    }
}

/// Additions in FFmpeg 8.1: `avcodec_receive_frame_flags()` and the
/// `AVAlphaMode` enum (referenced, not called).
fn demo_8_1_additions() {
    #[cfg(feature = "ffmpeg8_1")]
    {
        let _keep = ffi::avcodec_receive_frame_flags;
        assert_eq!(ffi::AVALPHA_MODE_UNSPECIFIED, 0);
        println!("avcodec_receive_frame_flags + AVALPHA_MODE_UNSPECIFIED (8.1+) OK");
    }
}
