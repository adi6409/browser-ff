#include "mozilla/StaticPrefs_accessibility.h"
#include "mozilla/StaticPrefs_beacon.h"
#include "mozilla/StaticPrefs_browser.h"
#include "mozilla/StaticPrefs_canvas.h"
#include "mozilla/StaticPrefs_dom.h"
#include "mozilla/StaticPrefs_geo.h"
#include "mozilla/StaticPrefs_gfx.h"
#include "mozilla/StaticPrefs_html5.h"
#include "mozilla/StaticPrefs_javascript.h"
#include "mozilla/StaticPrefs_layout.h"
#include "mozilla/StaticPrefs_media.h"
#include "mozilla/StaticPrefs_security.h"
#include "mozilla/dom/WebIDLPrefs.h"

namespace mozilla {
namespace dom {
const WebIDLPrefFunc sWebIDLPrefs[] = {
  nullptr,
  // accessibility.ARIAReflection.enabled
  StaticPrefs::accessibility_ARIAReflection_enabled,
  // beacon.enabled
  StaticPrefs::beacon_enabled,
  // browser.cache.offline.enable
  StaticPrefs::browser_cache_offline_enable,
  // canvas.capturestream.enabled
  StaticPrefs::canvas_capturestream_enabled,
  // canvas.createConicGradient.enabled
  StaticPrefs::canvas_createConicGradient_enabled,
  // canvas.filters.enabled
  StaticPrefs::canvas_filters_enabled,
  // canvas.focusring.enabled
  StaticPrefs::canvas_focusring_enabled,
  // canvas.hitregions.enabled
  StaticPrefs::canvas_hitregions_enabled,
  // canvas.mozgetasfile.enabled
  StaticPrefs::canvas_mozgetasfile_enabled,
  // dom.animations-api.autoremove.enabled
  StaticPrefs::dom_animations_api_autoremove_enabled,
  // dom.animations-api.compositing.enabled
  StaticPrefs::dom_animations_api_compositing_enabled,
  // dom.audioworklet.enabled
  StaticPrefs::dom_audioworklet_enabled,
  // dom.battery.enabled
  StaticPrefs::dom_battery_enabled,
  // dom.caches.enabled
  StaticPrefs::dom_caches_enabled,
  // dom.capture.enabled
  StaticPrefs::dom_capture_enabled,
  // dom.enable_event_timing
  StaticPrefs::dom_enable_event_timing,
  // dom.enable_memory_stats
  StaticPrefs::dom_enable_memory_stats,
  // dom.enable_window_print
  StaticPrefs::dom_enable_window_print,
  // dom.events.asyncClipboard
  StaticPrefs::dom_events_asyncClipboard,
  // dom.events.asyncClipboard.clipboardItem
  StaticPrefs::dom_events_asyncClipboard_clipboardItem,
  // dom.forms.autocapitalize
  StaticPrefs::dom_forms_autocapitalize,
  // dom.forms.enterkeyhint
  StaticPrefs::dom_forms_enterkeyhint,
  // dom.forms.inputmode
  StaticPrefs::dom_forms_inputmode,
  // dom.gamepad.enabled
  StaticPrefs::dom_gamepad_enabled,
  // dom.gamepad.extensions.enabled
  StaticPrefs::dom_gamepad_extensions_enabled,
  // dom.gamepad.extensions.lightindicator
  StaticPrefs::dom_gamepad_extensions_lightindicator,
  // dom.gamepad.extensions.multitouch
  StaticPrefs::dom_gamepad_extensions_multitouch,
  // dom.gamepad.test.enabled
  StaticPrefs::dom_gamepad_test_enabled,
  // dom.image-lazy-loading.enabled
  StaticPrefs::dom_image_lazy_loading_enabled,
  // dom.input.dirpicker
  StaticPrefs::dom_input_dirpicker,
  // dom.input_events.beforeinput.enabled
  StaticPrefs::dom_input_events_beforeinput_enabled,
  // dom.media.autoplay.autoplay-policy-api
  StaticPrefs::dom_media_autoplay_autoplay_policy_api,
  // dom.media.mediasession.enabled
  StaticPrefs::dom_media_mediasession_enabled,
  // dom.menuitem.enabled
  StaticPrefs::dom_menuitem_enabled,
  // dom.moduleScripts.enabled
  StaticPrefs::dom_moduleScripts_enabled,
  // dom.mozPaintCount.enabled
  StaticPrefs::dom_mozPaintCount_enabled,
  // dom.netinfo.enabled
  StaticPrefs::dom_netinfo_enabled,
  // dom.paintWorklet.enabled
  StaticPrefs::dom_paintWorklet_enabled,
  // dom.performance.time_to_contentful_paint.enabled
  StaticPrefs::dom_performance_time_to_contentful_paint_enabled,
  // dom.performance.time_to_dom_content_flushed.enabled
  StaticPrefs::dom_performance_time_to_dom_content_flushed_enabled,
  // dom.performance.time_to_first_interactive.enabled
  StaticPrefs::dom_performance_time_to_first_interactive_enabled,
  // dom.performance.time_to_non_blank_paint.enabled
  StaticPrefs::dom_performance_time_to_non_blank_paint_enabled,
  // dom.permissions.revoke.enable
  StaticPrefs::dom_permissions_revoke_enable,
  // dom.pointer-lock.enabled
  StaticPrefs::dom_pointer_lock_enabled,
  // dom.presentation.controller.enabled
  StaticPrefs::dom_presentation_controller_enabled,
  // dom.presentation.enabled
  StaticPrefs::dom_presentation_enabled,
  // dom.presentation.receiver.enabled
  StaticPrefs::dom_presentation_receiver_enabled,
  // dom.push.enabled
  StaticPrefs::dom_push_enabled,
  // dom.security.featurePolicy.webidl.enabled
  StaticPrefs::dom_security_featurePolicy_webidl_enabled,
  // dom.select_events.enabled
  StaticPrefs::dom_select_events_enabled,
  // dom.serviceWorkers.testing.enabled
  StaticPrefs::dom_serviceWorkers_testing_enabled,
  // dom.storage.testing
  StaticPrefs::dom_storage_testing,
  // dom.storageManager.enabled
  StaticPrefs::dom_storageManager_enabled,
  // dom.storage_access.enabled
  StaticPrefs::dom_storage_access_enabled,
  // dom.testing.selection.GetRangesForInterval
  StaticPrefs::dom_testing_selection_GetRangesForInterval,
  // dom.textMetrics.actualBoundingBox.enabled
  StaticPrefs::dom_textMetrics_actualBoundingBox_enabled,
  // dom.textMetrics.baselines.enabled
  StaticPrefs::dom_textMetrics_baselines_enabled,
  // dom.textMetrics.emHeight.enabled
  StaticPrefs::dom_textMetrics_emHeight_enabled,
  // dom.textMetrics.fontBoundingBox.enabled
  StaticPrefs::dom_textMetrics_fontBoundingBox_enabled,
  // dom.visualviewport.enabled
  StaticPrefs::dom_visualviewport_enabled,
  // dom.vr.enabled
  StaticPrefs::dom_vr_enabled,
  // dom.vr.puppet.enabled
  StaticPrefs::dom_vr_puppet_enabled,
  // dom.vr.webxr.enabled
  StaticPrefs::dom_vr_webxr_enabled,
  // dom.webcomponents.formAssociatedCustomElement.enabled
  StaticPrefs::dom_webcomponents_formAssociatedCustomElement_enabled,
  // dom.webdriver.enabled
  StaticPrefs::dom_webdriver_enabled,
  // dom.webgpu.enabled
  StaticPrefs::dom_webgpu_enabled,
  // dom.webidl.test1
  StaticPrefs::dom_webidl_test1,
  // dom.webidl.test2
  StaticPrefs::dom_webidl_test2,
  // dom.webkitBlink.dirPicker.enabled
  StaticPrefs::dom_webkitBlink_dirPicker_enabled,
  // dom.webkitBlink.filesystem.enabled
  StaticPrefs::dom_webkitBlink_filesystem_enabled,
  // dom.webmidi.enabled
  StaticPrefs::dom_webmidi_enabled,
  // dom.webnotifications.requireinteraction.enabled
  StaticPrefs::dom_webnotifications_requireinteraction_enabled,
  // dom.webnotifications.serviceworker.enabled
  StaticPrefs::dom_webnotifications_serviceworker_enabled,
  // dom.window.event.enabled
  StaticPrefs::dom_window_event_enabled,
  // geo.enabled
  StaticPrefs::geo_enabled,
  // gfx.offscreencanvas.enabled
  StaticPrefs::gfx_offscreencanvas_enabled,
  // html5.inert.enabled
  StaticPrefs::html5_inert_enabled,
  // javascript.options.streams
  StaticPrefs::javascript_options_streams,
  // layout.css.aspect-ratio.enabled
  StaticPrefs::layout_css_aspect_ratio_enabled,
  // layout.css.constructable-stylesheets.enabled
  StaticPrefs::layout_css_constructable_stylesheets_enabled,
  // layout.css.convertFromNode.enabled
  StaticPrefs::layout_css_convertFromNode_enabled,
  // layout.css.font-display.enabled
  StaticPrefs::layout_css_font_display_enabled,
  // layout.css.font-loading-api.enabled
  StaticPrefs::layout_css_font_loading_api_enabled,
  // layout.css.font-variations.enabled
  StaticPrefs::layout_css_font_variations_enabled,
  // layout.css.grid-template-masonry-value.enabled
  StaticPrefs::layout_css_grid_template_masonry_value_enabled,
  // layout.css.individual-transform.enabled
  StaticPrefs::layout_css_individual_transform_enabled,
  // layout.css.initial-letter.enabled
  StaticPrefs::layout_css_initial_letter_enabled,
  // layout.css.math-depth.enabled
  StaticPrefs::layout_css_math_depth_enabled,
  // layout.css.math-style.enabled
  StaticPrefs::layout_css_math_style_enabled,
  // layout.css.motion-path.enabled
  StaticPrefs::layout_css_motion_path_enabled,
  // layout.css.osx-font-smoothing.enabled
  StaticPrefs::layout_css_osx_font_smoothing_enabled,
  // layout.css.overflow-clip-box.enabled
  StaticPrefs::layout_css_overflow_clip_box_enabled,
  // layout.css.overflow-logical.enabled
  StaticPrefs::layout_css_overflow_logical_enabled,
  // layout.css.overscroll-behavior.enabled
  StaticPrefs::layout_css_overscroll_behavior_enabled,
  // layout.css.prefixes.animations
  StaticPrefs::layout_css_prefixes_animations,
  // layout.css.prefixes.border-image
  StaticPrefs::layout_css_prefixes_border_image,
  // layout.css.prefixes.box-sizing
  StaticPrefs::layout_css_prefixes_box_sizing,
  // layout.css.prefixes.columns
  StaticPrefs::layout_css_prefixes_columns,
  // layout.css.prefixes.font-features
  StaticPrefs::layout_css_prefixes_font_features,
  // layout.css.prefixes.transforms
  StaticPrefs::layout_css_prefixes_transforms,
  // layout.css.prefixes.transitions
  StaticPrefs::layout_css_prefixes_transitions,
  // layout.css.scroll-anchoring.enabled
  StaticPrefs::layout_css_scroll_anchoring_enabled,
  // layout.css.touch_action.enabled
  StaticPrefs::layout_css_touch_action_enabled,
  // layout.css.webkit-line-clamp.enabled
  StaticPrefs::layout_css_webkit_line_clamp_enabled,
  // layout.css.zoom-transform-hack.enabled
  StaticPrefs::layout_css_zoom_transform_hack_enabled,
  // media.allowed-to-play.enabled
  StaticPrefs::media_allowed_to_play_enabled,
  // media.eme.hdcp-policy-check.enabled
  StaticPrefs::media_eme_hdcp_policy_check_enabled,
  // media.getdisplaymedia.enabled
  StaticPrefs::media_getdisplaymedia_enabled,
  // media.mediasource.enabled
  StaticPrefs::media_mediasource_enabled,
  // media.mediasource.experimental.enabled
  StaticPrefs::media_mediasource_experimental_enabled,
  // media.ondevicechange.enabled
  StaticPrefs::media_ondevicechange_enabled,
  // media.peerconnection.dtmf.enabled
  StaticPrefs::media_peerconnection_dtmf_enabled,
  // media.peerconnection.identity.enabled
  StaticPrefs::media_peerconnection_identity_enabled,
  // media.peerconnection.rtpsourcesapi.enabled
  StaticPrefs::media_peerconnection_rtpsourcesapi_enabled,
  // media.seekToNextFrame.enabled
  StaticPrefs::media_seekToNextFrame_enabled,
  // media.setsinkid.enabled
  StaticPrefs::media_setsinkid_enabled,
  // media.test.video-suspend
  StaticPrefs::media_test_video_suspend,
  // media.track.enabled
  StaticPrefs::media_track_enabled,
  // media.useAudioChannelService.testing
  StaticPrefs::media_useAudioChannelService_testing,
  // media.videocontrols.lock-video-orientation
  StaticPrefs::media_videocontrols_lock_video_orientation,
  // media.webspeech.synth.enabled
  StaticPrefs::media_webspeech_synth_enabled,
  // media.webvtt.regions.enabled
  StaticPrefs::media_webvtt_regions_enabled,
  // security.webauth.u2f
  StaticPrefs::security_webauth_u2f,
  // security.webauth.webauthn
  StaticPrefs::security_webauth_webauthn
};
} // namespace dom
} // namespace mozilla

