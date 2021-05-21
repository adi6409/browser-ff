#ifndef mozilla_dom_WebIDLPrefs_h
#define mozilla_dom_WebIDLPrefs_h


namespace mozilla {
namespace dom {
enum class WebIDLPrefIndex : uint8_t {
  NoPref,
  // accessibility.ARIAReflection.enabled
  accessibility_ARIAReflection_enabled,
  // beacon.enabled
  beacon_enabled,
  // browser.cache.offline.enable
  browser_cache_offline_enable,
  // canvas.capturestream.enabled
  canvas_capturestream_enabled,
  // canvas.createConicGradient.enabled
  canvas_createConicGradient_enabled,
  // canvas.filters.enabled
  canvas_filters_enabled,
  // canvas.focusring.enabled
  canvas_focusring_enabled,
  // canvas.hitregions.enabled
  canvas_hitregions_enabled,
  // canvas.mozgetasfile.enabled
  canvas_mozgetasfile_enabled,
  // dom.animations-api.autoremove.enabled
  dom_animations_api_autoremove_enabled,
  // dom.animations-api.compositing.enabled
  dom_animations_api_compositing_enabled,
  // dom.audioworklet.enabled
  dom_audioworklet_enabled,
  // dom.battery.enabled
  dom_battery_enabled,
  // dom.caches.enabled
  dom_caches_enabled,
  // dom.capture.enabled
  dom_capture_enabled,
  // dom.enable_event_timing
  dom_enable_event_timing,
  // dom.enable_memory_stats
  dom_enable_memory_stats,
  // dom.enable_window_print
  dom_enable_window_print,
  // dom.events.asyncClipboard
  dom_events_asyncClipboard,
  // dom.events.asyncClipboard.clipboardItem
  dom_events_asyncClipboard_clipboardItem,
  // dom.forms.autocapitalize
  dom_forms_autocapitalize,
  // dom.forms.enterkeyhint
  dom_forms_enterkeyhint,
  // dom.forms.inputmode
  dom_forms_inputmode,
  // dom.gamepad.enabled
  dom_gamepad_enabled,
  // dom.gamepad.extensions.enabled
  dom_gamepad_extensions_enabled,
  // dom.gamepad.extensions.lightindicator
  dom_gamepad_extensions_lightindicator,
  // dom.gamepad.extensions.multitouch
  dom_gamepad_extensions_multitouch,
  // dom.gamepad.test.enabled
  dom_gamepad_test_enabled,
  // dom.image-lazy-loading.enabled
  dom_image_lazy_loading_enabled,
  // dom.input.dirpicker
  dom_input_dirpicker,
  // dom.input_events.beforeinput.enabled
  dom_input_events_beforeinput_enabled,
  // dom.media.autoplay.autoplay-policy-api
  dom_media_autoplay_autoplay_policy_api,
  // dom.media.mediasession.enabled
  dom_media_mediasession_enabled,
  // dom.menuitem.enabled
  dom_menuitem_enabled,
  // dom.moduleScripts.enabled
  dom_moduleScripts_enabled,
  // dom.mozPaintCount.enabled
  dom_mozPaintCount_enabled,
  // dom.netinfo.enabled
  dom_netinfo_enabled,
  // dom.paintWorklet.enabled
  dom_paintWorklet_enabled,
  // dom.performance.time_to_contentful_paint.enabled
  dom_performance_time_to_contentful_paint_enabled,
  // dom.performance.time_to_dom_content_flushed.enabled
  dom_performance_time_to_dom_content_flushed_enabled,
  // dom.performance.time_to_first_interactive.enabled
  dom_performance_time_to_first_interactive_enabled,
  // dom.performance.time_to_non_blank_paint.enabled
  dom_performance_time_to_non_blank_paint_enabled,
  // dom.permissions.revoke.enable
  dom_permissions_revoke_enable,
  // dom.pointer-lock.enabled
  dom_pointer_lock_enabled,
  // dom.presentation.controller.enabled
  dom_presentation_controller_enabled,
  // dom.presentation.enabled
  dom_presentation_enabled,
  // dom.presentation.receiver.enabled
  dom_presentation_receiver_enabled,
  // dom.push.enabled
  dom_push_enabled,
  // dom.security.featurePolicy.webidl.enabled
  dom_security_featurePolicy_webidl_enabled,
  // dom.select_events.enabled
  dom_select_events_enabled,
  // dom.serviceWorkers.testing.enabled
  dom_serviceWorkers_testing_enabled,
  // dom.storage.testing
  dom_storage_testing,
  // dom.storageManager.enabled
  dom_storageManager_enabled,
  // dom.storage_access.enabled
  dom_storage_access_enabled,
  // dom.testing.selection.GetRangesForInterval
  dom_testing_selection_GetRangesForInterval,
  // dom.textMetrics.actualBoundingBox.enabled
  dom_textMetrics_actualBoundingBox_enabled,
  // dom.textMetrics.baselines.enabled
  dom_textMetrics_baselines_enabled,
  // dom.textMetrics.emHeight.enabled
  dom_textMetrics_emHeight_enabled,
  // dom.textMetrics.fontBoundingBox.enabled
  dom_textMetrics_fontBoundingBox_enabled,
  // dom.visualviewport.enabled
  dom_visualviewport_enabled,
  // dom.vr.enabled
  dom_vr_enabled,
  // dom.vr.puppet.enabled
  dom_vr_puppet_enabled,
  // dom.vr.webxr.enabled
  dom_vr_webxr_enabled,
  // dom.webcomponents.formAssociatedCustomElement.enabled
  dom_webcomponents_formAssociatedCustomElement_enabled,
  // dom.webdriver.enabled
  dom_webdriver_enabled,
  // dom.webgpu.enabled
  dom_webgpu_enabled,
  // dom.webidl.test1
  dom_webidl_test1,
  // dom.webidl.test2
  dom_webidl_test2,
  // dom.webkitBlink.dirPicker.enabled
  dom_webkitBlink_dirPicker_enabled,
  // dom.webkitBlink.filesystem.enabled
  dom_webkitBlink_filesystem_enabled,
  // dom.webmidi.enabled
  dom_webmidi_enabled,
  // dom.webnotifications.requireinteraction.enabled
  dom_webnotifications_requireinteraction_enabled,
  // dom.webnotifications.serviceworker.enabled
  dom_webnotifications_serviceworker_enabled,
  // dom.window.event.enabled
  dom_window_event_enabled,
  // geo.enabled
  geo_enabled,
  // gfx.offscreencanvas.enabled
  gfx_offscreencanvas_enabled,
  // html5.inert.enabled
  html5_inert_enabled,
  // javascript.options.streams
  javascript_options_streams,
  // layout.css.aspect-ratio.enabled
  layout_css_aspect_ratio_enabled,
  // layout.css.constructable-stylesheets.enabled
  layout_css_constructable_stylesheets_enabled,
  // layout.css.convertFromNode.enabled
  layout_css_convertFromNode_enabled,
  // layout.css.font-display.enabled
  layout_css_font_display_enabled,
  // layout.css.font-loading-api.enabled
  layout_css_font_loading_api_enabled,
  // layout.css.font-variations.enabled
  layout_css_font_variations_enabled,
  // layout.css.grid-template-masonry-value.enabled
  layout_css_grid_template_masonry_value_enabled,
  // layout.css.individual-transform.enabled
  layout_css_individual_transform_enabled,
  // layout.css.initial-letter.enabled
  layout_css_initial_letter_enabled,
  // layout.css.math-depth.enabled
  layout_css_math_depth_enabled,
  // layout.css.math-style.enabled
  layout_css_math_style_enabled,
  // layout.css.motion-path.enabled
  layout_css_motion_path_enabled,
  // layout.css.osx-font-smoothing.enabled
  layout_css_osx_font_smoothing_enabled,
  // layout.css.overflow-clip-box.enabled
  layout_css_overflow_clip_box_enabled,
  // layout.css.overflow-logical.enabled
  layout_css_overflow_logical_enabled,
  // layout.css.overscroll-behavior.enabled
  layout_css_overscroll_behavior_enabled,
  // layout.css.prefixes.animations
  layout_css_prefixes_animations,
  // layout.css.prefixes.border-image
  layout_css_prefixes_border_image,
  // layout.css.prefixes.box-sizing
  layout_css_prefixes_box_sizing,
  // layout.css.prefixes.columns
  layout_css_prefixes_columns,
  // layout.css.prefixes.font-features
  layout_css_prefixes_font_features,
  // layout.css.prefixes.transforms
  layout_css_prefixes_transforms,
  // layout.css.prefixes.transitions
  layout_css_prefixes_transitions,
  // layout.css.scroll-anchoring.enabled
  layout_css_scroll_anchoring_enabled,
  // layout.css.touch_action.enabled
  layout_css_touch_action_enabled,
  // layout.css.webkit-line-clamp.enabled
  layout_css_webkit_line_clamp_enabled,
  // layout.css.zoom-transform-hack.enabled
  layout_css_zoom_transform_hack_enabled,
  // media.allowed-to-play.enabled
  media_allowed_to_play_enabled,
  // media.eme.hdcp-policy-check.enabled
  media_eme_hdcp_policy_check_enabled,
  // media.getdisplaymedia.enabled
  media_getdisplaymedia_enabled,
  // media.mediasource.enabled
  media_mediasource_enabled,
  // media.mediasource.experimental.enabled
  media_mediasource_experimental_enabled,
  // media.ondevicechange.enabled
  media_ondevicechange_enabled,
  // media.peerconnection.dtmf.enabled
  media_peerconnection_dtmf_enabled,
  // media.peerconnection.identity.enabled
  media_peerconnection_identity_enabled,
  // media.peerconnection.rtpsourcesapi.enabled
  media_peerconnection_rtpsourcesapi_enabled,
  // media.seekToNextFrame.enabled
  media_seekToNextFrame_enabled,
  // media.setsinkid.enabled
  media_setsinkid_enabled,
  // media.test.video-suspend
  media_test_video_suspend,
  // media.track.enabled
  media_track_enabled,
  // media.useAudioChannelService.testing
  media_useAudioChannelService_testing,
  // media.videocontrols.lock-video-orientation
  media_videocontrols_lock_video_orientation,
  // media.webspeech.synth.enabled
  media_webspeech_synth_enabled,
  // media.webvtt.regions.enabled
  media_webvtt_regions_enabled,
  // security.webauth.u2f
  security_webauth_u2f,
  // security.webauth.webauthn
  security_webauth_webauthn
};
typedef bool (*WebIDLPrefFunc)();
extern const WebIDLPrefFunc sWebIDLPrefs[125];
} // namespace dom
} // namespace mozilla


#endif // mozilla_dom_WebIDLPrefs_h
