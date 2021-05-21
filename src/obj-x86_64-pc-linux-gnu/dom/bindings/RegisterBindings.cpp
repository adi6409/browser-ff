#include "APZTestDataBinding.h"
#include "AbortControllerBinding.h"
#include "AbortSignalBinding.h"
#include "AbstractRangeBinding.h"
#include "AccessibleNodeBinding.h"
#include "AddonEventBinding.h"
#include "AddonManagerBinding.h"
#include "AnalyserNodeBinding.h"
#include "AnimationBinding.h"
#include "AnimationEffectBinding.h"
#include "AnimationEventBinding.h"
#include "AnimationPlaybackEventBinding.h"
#include "AnimationTimelineBinding.h"
#include "AnonymousContentBinding.h"
#include "AttrBinding.h"
#include "AudioBufferBinding.h"
#include "AudioBufferSourceNodeBinding.h"
#include "AudioContextBinding.h"
#include "AudioDestinationNodeBinding.h"
#include "AudioListenerBinding.h"
#include "AudioNodeBinding.h"
#include "AudioParamBinding.h"
#include "AudioParamMapBinding.h"
#include "AudioProcessingEventBinding.h"
#include "AudioScheduledSourceNodeBinding.h"
#include "AudioTrackBinding.h"
#include "AudioTrackListBinding.h"
#include "AudioWorkletBinding.h"
#include "AudioWorkletNodeBinding.h"
#include "BarPropBinding.h"
#include "BaseAudioContextBinding.h"
#include "BatteryManagerBinding.h"
#include "BeforeUnloadEventBinding.h"
#include "BiquadFilterNodeBinding.h"
#include "BlobBinding.h"
#include "BlobEventBinding.h"
#include "BroadcastChannelBinding.h"
#include "BrowsingContextBinding.h"
#include "CDATASectionBinding.h"
#include "CSS2PropertiesBinding.h"
#include "CSSAnimationBinding.h"
#include "CSSBinding.h"
#include "CSSConditionRuleBinding.h"
#include "CSSCounterStyleRuleBinding.h"
#include "CSSFontFaceRuleBinding.h"
#include "CSSFontFeatureValuesRuleBinding.h"
#include "CSSGroupingRuleBinding.h"
#include "CSSImportRuleBinding.h"
#include "CSSKeyframeRuleBinding.h"
#include "CSSKeyframesRuleBinding.h"
#include "CSSMediaRuleBinding.h"
#include "CSSMozDocumentRuleBinding.h"
#include "CSSNamespaceRuleBinding.h"
#include "CSSPageRuleBinding.h"
#include "CSSPseudoElementBinding.h"
#include "CSSRuleBinding.h"
#include "CSSRuleListBinding.h"
#include "CSSStyleDeclarationBinding.h"
#include "CSSStyleRuleBinding.h"
#include "CSSStyleSheetBinding.h"
#include "CSSSupportsRuleBinding.h"
#include "CSSTransitionBinding.h"
#include "CacheBinding.h"
#include "CacheStorageBinding.h"
#include "CanvasCaptureMediaStreamBinding.h"
#include "CanvasRenderingContext2DBinding.h"
#include "CaretPositionBinding.h"
#include "CaretStateChangedEventBinding.h"
#include "ChannelMergerNodeBinding.h"
#include "ChannelSplitterNodeBinding.h"
#include "ChannelWrapperBinding.h"
#include "CharacterDataBinding.h"
#include "CheckerboardReportServiceBinding.h"
#include "ChildSHistoryBinding.h"
#include "ChromeNodeListBinding.h"
#include "ChromeUtilsBinding.h"
#include "ClipboardBinding.h"
#include "ClipboardEventBinding.h"
#include "ClonedErrorHolderBinding.h"
#include "CloseEventBinding.h"
#include "CommandEventBinding.h"
#include "CommentBinding.h"
#include "CompositionEventBinding.h"
#include "ConsoleBinding.h"
#include "ConstantSourceNodeBinding.h"
#include "ConvolverNodeBinding.h"
#include "CreateOfferRequestBinding.h"
#include "CredentialManagementBinding.h"
#include "CryptoBinding.h"
#include "CustomElementRegistryBinding.h"
#include "CustomEventBinding.h"
#include "DOMExceptionBinding.h"
#include "DOMImplementationBinding.h"
#include "DOMLocalizationBinding.h"
#include "DOMMatrixBinding.h"
#include "DOMParserBinding.h"
#include "DOMPointBinding.h"
#include "DOMQuadBinding.h"
#include "DOMRectBinding.h"
#include "DOMRectListBinding.h"
#include "DOMRequestBinding.h"
#include "DOMStringListBinding.h"
#include "DOMStringMapBinding.h"
#include "DOMTokenListBinding.h"
#include "DataTransferBinding.h"
#include "DataTransferItemBinding.h"
#include "DataTransferItemListBinding.h"
#include "DebuggerNotificationBinding.h"
#include "DebuggerNotificationObserverBinding.h"
#include "DelayNodeBinding.h"
#include "DeviceLightEventBinding.h"
#include "DeviceMotionEventBinding.h"
#include "DeviceOrientationEventBinding.h"
#include "DeviceProximityEventBinding.h"
#include "DirectoryBinding.h"
#include "DocumentBinding.h"
#include "DocumentFragmentBinding.h"
#include "DocumentTimelineBinding.h"
#include "DocumentTypeBinding.h"
#include "DominatorTreeBinding.h"
#include "DragEventBinding.h"
#include "DynamicsCompressorNodeBinding.h"
#include "ElementBinding.h"
#include "ElementInternalsBinding.h"
#include "ErrorEventBinding.h"
#include "EventBinding.h"
#include "EventSourceBinding.h"
#include "EventTargetBinding.h"
#include "FeaturePolicyBinding.h"
#include "FetchObserverBinding.h"
#include "FileBinding.h"
#include "FileListBinding.h"
#include "FileReaderBinding.h"
#include "FileSystemBinding.h"
#include "FileSystemDirectoryEntryBinding.h"
#include "FileSystemDirectoryReaderBinding.h"
#include "FileSystemEntryBinding.h"
#include "FileSystemFileEntryBinding.h"
#include "FlexBinding.h"
#include "FluentBinding.h"
#include "FocusEventBinding.h"
#include "FontFaceBinding.h"
#include "FontFaceSetBinding.h"
#include "FontFaceSetLoadEventBinding.h"
#include "FormDataBinding.h"
#include "FormDataEventBinding.h"
#include "FrameCrashedEventBinding.h"
#include "FrameLoaderBinding.h"
#include "GPUUncapturedErrorEventBinding.h"
#include "GainNodeBinding.h"
#include "GamepadAxisMoveEventBinding.h"
#include "GamepadBinding.h"
#include "GamepadButtonEventBinding.h"
#include "GamepadEventBinding.h"
#include "GamepadHapticActuatorBinding.h"
#include "GamepadLightIndicatorBinding.h"
#include "GamepadPoseBinding.h"
#include "GamepadServiceTestBinding.h"
#include "GamepadTouchBinding.h"
#include "GeolocationBinding.h"
#include "GeolocationCoordinatesBinding.h"
#include "GeolocationPositionBinding.h"
#include "GeolocationPositionErrorBinding.h"
#include "GleanBinding.h"
#include "GleanPingsBinding.h"
#include "GridBinding.h"
#include "HTMLAllCollectionBinding.h"
#include "HTMLAnchorElementBinding.h"
#include "HTMLAreaElementBinding.h"
#include "HTMLAudioElementBinding.h"
#include "HTMLBRElementBinding.h"
#include "HTMLBaseElementBinding.h"
#include "HTMLBodyElementBinding.h"
#include "HTMLButtonElementBinding.h"
#include "HTMLCanvasElementBinding.h"
#include "HTMLCollectionBinding.h"
#include "HTMLDListElementBinding.h"
#include "HTMLDataElementBinding.h"
#include "HTMLDataListElementBinding.h"
#include "HTMLDetailsElementBinding.h"
#include "HTMLDialogElementBinding.h"
#include "HTMLDirectoryElementBinding.h"
#include "HTMLDivElementBinding.h"
#include "HTMLDocumentBinding.h"
#include "HTMLElementBinding.h"
#include "HTMLEmbedElementBinding.h"
#include "HTMLFieldSetElementBinding.h"
#include "HTMLFontElementBinding.h"
#include "HTMLFormControlsCollectionBinding.h"
#include "HTMLFormElementBinding.h"
#include "HTMLFrameElementBinding.h"
#include "HTMLFrameSetElementBinding.h"
#include "HTMLHRElementBinding.h"
#include "HTMLHeadElementBinding.h"
#include "HTMLHeadingElementBinding.h"
#include "HTMLHtmlElementBinding.h"
#include "HTMLIFrameElementBinding.h"
#include "HTMLImageElementBinding.h"
#include "HTMLInputElementBinding.h"
#include "HTMLLIElementBinding.h"
#include "HTMLLabelElementBinding.h"
#include "HTMLLegendElementBinding.h"
#include "HTMLLinkElementBinding.h"
#include "HTMLMapElementBinding.h"
#include "HTMLMarqueeElementBinding.h"
#include "HTMLMediaElementBinding.h"
#include "HTMLMenuElementBinding.h"
#include "HTMLMenuItemElementBinding.h"
#include "HTMLMetaElementBinding.h"
#include "HTMLMeterElementBinding.h"
#include "HTMLModElementBinding.h"
#include "HTMLOListElementBinding.h"
#include "HTMLObjectElementBinding.h"
#include "HTMLOptGroupElementBinding.h"
#include "HTMLOptionElementBinding.h"
#include "HTMLOptionsCollectionBinding.h"
#include "HTMLOutputElementBinding.h"
#include "HTMLParagraphElementBinding.h"
#include "HTMLParamElementBinding.h"
#include "HTMLPictureElementBinding.h"
#include "HTMLPreElementBinding.h"
#include "HTMLProgressElementBinding.h"
#include "HTMLQuoteElementBinding.h"
#include "HTMLScriptElementBinding.h"
#include "HTMLSelectElementBinding.h"
#include "HTMLSlotElementBinding.h"
#include "HTMLSourceElementBinding.h"
#include "HTMLSpanElementBinding.h"
#include "HTMLStyleElementBinding.h"
#include "HTMLTableCaptionElementBinding.h"
#include "HTMLTableCellElementBinding.h"
#include "HTMLTableColElementBinding.h"
#include "HTMLTableElementBinding.h"
#include "HTMLTableRowElementBinding.h"
#include "HTMLTableSectionElementBinding.h"
#include "HTMLTemplateElementBinding.h"
#include "HTMLTextAreaElementBinding.h"
#include "HTMLTimeElementBinding.h"
#include "HTMLTitleElementBinding.h"
#include "HTMLTrackElementBinding.h"
#include "HTMLUListElementBinding.h"
#include "HTMLVideoElementBinding.h"
#include "HashChangeEventBinding.h"
#include "HeadersBinding.h"
#include "HeapSnapshotBinding.h"
#include "HiddenPluginEventBinding.h"
#include "HistoryBinding.h"
#include "IDBCursorBinding.h"
#include "IDBDatabaseBinding.h"
#include "IDBFactoryBinding.h"
#include "IDBFileHandleBinding.h"
#include "IDBFileRequestBinding.h"
#include "IDBIndexBinding.h"
#include "IDBKeyRangeBinding.h"
#include "IDBMutableFileBinding.h"
#include "IDBObjectStoreBinding.h"
#include "IDBOpenDBRequestBinding.h"
#include "IDBRequestBinding.h"
#include "IDBTransactionBinding.h"
#include "IDBVersionChangeEventBinding.h"
#include "IIRFilterNodeBinding.h"
#include "IOUtilsBinding.h"
#include "IdleDeadlineBinding.h"
#include "ImageBitmapBinding.h"
#include "ImageBitmapRenderingContextBinding.h"
#include "ImageCaptureBinding.h"
#include "ImageCaptureErrorEventBinding.h"
#include "ImageDataBinding.h"
#include "ImageDocumentBinding.h"
#include "InputEventBinding.h"
#include "InspectorUtilsBinding.h"
#include "InstallTriggerBinding.h"
#include "IntersectionObserverBinding.h"
#include "JSProcessActorBinding.h"
#include "JSWindowActorBinding.h"
#include "KeyEventBinding.h"
#include "KeyboardEventBinding.h"
#include "KeyframeEffectBinding.h"
#include "L10nOverlaysBinding.h"
#include "LocalizationBinding.h"
#include "LocationBinding.h"
#include "MIDIAccessBinding.h"
#include "MIDIConnectionEventBinding.h"
#include "MIDIInputBinding.h"
#include "MIDIInputMapBinding.h"
#include "MIDIMessageEventBinding.h"
#include "MIDIOutputBinding.h"
#include "MIDIOutputMapBinding.h"
#include "MIDIPortBinding.h"
#include "MatchGlobBinding.h"
#include "MatchPatternBinding.h"
#include "MathMLElementBinding.h"
#include "MediaCapabilitiesBinding.h"
#include "MediaControllerBinding.h"
#include "MediaDeviceInfoBinding.h"
#include "MediaDevicesBinding.h"
#include "MediaElementAudioSourceNodeBinding.h"
#include "MediaEncryptedEventBinding.h"
#include "MediaErrorBinding.h"
#include "MediaKeyErrorBinding.h"
#include "MediaKeyMessageEventBinding.h"
#include "MediaKeySessionBinding.h"
#include "MediaKeyStatusMapBinding.h"
#include "MediaKeySystemAccessBinding.h"
#include "MediaKeysBinding.h"
#include "MediaListBinding.h"
#include "MediaQueryListBinding.h"
#include "MediaQueryListEventBinding.h"
#include "MediaRecorderBinding.h"
#include "MediaRecorderErrorEventBinding.h"
#include "MediaSessionBinding.h"
#include "MediaSourceBinding.h"
#include "MediaStreamAudioDestinationNodeBinding.h"
#include "MediaStreamAudioSourceNodeBinding.h"
#include "MediaStreamBinding.h"
#include "MediaStreamEventBinding.h"
#include "MediaStreamTrackAudioSourceNodeBinding.h"
#include "MediaStreamTrackBinding.h"
#include "MediaStreamTrackEventBinding.h"
#include "MerchantValidationEventBinding.h"
#include "MessageChannelBinding.h"
#include "MessageEventBinding.h"
#include "MessageManagerBinding.h"
#include "MessagePortBinding.h"
#include "MimeTypeArrayBinding.h"
#include "MimeTypeBinding.h"
#include "MouseEventBinding.h"
#include "MouseScrollEventBinding.h"
#include "MozDocumentObserverBinding.h"
#include "MozSharedMapBinding.h"
#include "MozStorageAsyncStatementParamsBinding.h"
#include "MozStorageStatementParamsBinding.h"
#include "MozStorageStatementRowBinding.h"
#include "MutationEventBinding.h"
#include "MutationObserverBinding.h"
#include "NamedNodeMapBinding.h"
#include "NavigatorBinding.h"
#include "NetworkInformationBinding.h"
#include "NodeBinding.h"
#include "NodeFilterBinding.h"
#include "NodeIteratorBinding.h"
#include "NodeListBinding.h"
#include "NotificationBinding.h"
#include "NotifyPaintEventBinding.h"
#include "OfflineAudioCompletionEventBinding.h"
#include "OfflineAudioContextBinding.h"
#include "OfflineResourceListBinding.h"
#include "OffscreenCanvasBinding.h"
#include "OscillatorNodeBinding.h"
#include "PageTransitionEventBinding.h"
#include "PaintRequestBinding.h"
#include "PaintRequestListBinding.h"
#include "PannerNodeBinding.h"
#include "PathUtilsBinding.h"
#include "PaymentAddressBinding.h"
#include "PaymentMethodChangeEventBinding.h"
#include "PaymentRequestBinding.h"
#include "PaymentRequestUpdateEventBinding.h"
#include "PaymentResponseBinding.h"
#include "PeerConnectionImplBinding.h"
#include "PeerConnectionObserverBinding.h"
#include "PerformanceBinding.h"
#include "PerformanceEntryBinding.h"
#include "PerformanceEntryEventBinding.h"
#include "PerformanceEventTimingBinding.h"
#include "PerformanceMarkBinding.h"
#include "PerformanceMeasureBinding.h"
#include "PerformanceNavigationBinding.h"
#include "PerformanceNavigationTimingBinding.h"
#include "PerformanceObserverBinding.h"
#include "PerformanceObserverEntryListBinding.h"
#include "PerformancePaintTimingBinding.h"
#include "PerformanceResourceTimingBinding.h"
#include "PerformanceServerTimingBinding.h"
#include "PerformanceTimingBinding.h"
#include "PeriodicWaveBinding.h"
#include "PermissionStatusBinding.h"
#include "PermissionsBinding.h"
#include "PlacesEventBinding.h"
#include "PlacesObserversBinding.h"
#include "PluginArrayBinding.h"
#include "PluginBinding.h"
#include "PluginCrashedEventBinding.h"
#include "PointerEventBinding.h"
#include "PopStateEventBinding.h"
#include "PopupBlockedEventBinding.h"
#include "PopupPositionedEventBinding.h"
#include "PositionStateEventBinding.h"
#include "PrecompiledScriptBinding.h"
#include "PresentationAvailabilityBinding.h"
#include "PresentationBinding.h"
#include "PresentationConnectionAvailableEventBinding.h"
#include "PresentationConnectionBinding.h"
#include "PresentationConnectionCloseEventBinding.h"
#include "PresentationConnectionListBinding.h"
#include "PresentationReceiverBinding.h"
#include "PresentationRequestBinding.h"
#include "PrioEncoderBinding.h"
#include "ProcessingInstructionBinding.h"
#include "ProgressEventBinding.h"
#include "PromiseDebuggingBinding.h"
#include "PromiseRejectionEventBinding.h"
#include "PushManagerBinding.h"
#include "PushSubscriptionBinding.h"
#include "PushSubscriptionOptionsBinding.h"
#include "RTCCertificateBinding.h"
#include "RTCDTMFSenderBinding.h"
#include "RTCDTMFToneChangeEventBinding.h"
#include "RTCDataChannelBinding.h"
#include "RTCDataChannelEventBinding.h"
#include "RTCDtlsTransportBinding.h"
#include "RTCIceCandidateBinding.h"
#include "RTCPeerConnectionBinding.h"
#include "RTCPeerConnectionIceEventBinding.h"
#include "RTCPeerConnectionStaticBinding.h"
#include "RTCRtpReceiverBinding.h"
#include "RTCRtpSenderBinding.h"
#include "RTCRtpTransceiverBinding.h"
#include "RTCSessionDescriptionBinding.h"
#include "RTCStatsReportBinding.h"
#include "RTCTrackEventBinding.h"
#include "RadioNodeListBinding.h"
#include "RangeBinding.h"
#include "ReportingBinding.h"
#include "RequestBinding.h"
#include "ResizeObserverBinding.h"
#include "ResponseBinding.h"
#include "SVGAElementBinding.h"
#include "SVGAngleBinding.h"
#include "SVGAnimateElementBinding.h"
#include "SVGAnimateMotionElementBinding.h"
#include "SVGAnimateTransformElementBinding.h"
#include "SVGAnimatedAngleBinding.h"
#include "SVGAnimatedBooleanBinding.h"
#include "SVGAnimatedEnumerationBinding.h"
#include "SVGAnimatedIntegerBinding.h"
#include "SVGAnimatedLengthBinding.h"
#include "SVGAnimatedLengthListBinding.h"
#include "SVGAnimatedNumberBinding.h"
#include "SVGAnimatedNumberListBinding.h"
#include "SVGAnimatedPreserveAspectRatioBinding.h"
#include "SVGAnimatedRectBinding.h"
#include "SVGAnimatedStringBinding.h"
#include "SVGAnimatedTransformListBinding.h"
#include "SVGAnimationElementBinding.h"
#include "SVGCircleElementBinding.h"
#include "SVGClipPathElementBinding.h"
#include "SVGComponentTransferFunctionElementBinding.h"
#include "SVGDefsElementBinding.h"
#include "SVGDescElementBinding.h"
#include "SVGElementBinding.h"
#include "SVGEllipseElementBinding.h"
#include "SVGFEBlendElementBinding.h"
#include "SVGFEColorMatrixElementBinding.h"
#include "SVGFEComponentTransferElementBinding.h"
#include "SVGFECompositeElementBinding.h"
#include "SVGFEConvolveMatrixElementBinding.h"
#include "SVGFEDiffuseLightingElementBinding.h"
#include "SVGFEDisplacementMapElementBinding.h"
#include "SVGFEDistantLightElementBinding.h"
#include "SVGFEDropShadowElementBinding.h"
#include "SVGFEFloodElementBinding.h"
#include "SVGFEFuncAElementBinding.h"
#include "SVGFEFuncBElementBinding.h"
#include "SVGFEFuncGElementBinding.h"
#include "SVGFEFuncRElementBinding.h"
#include "SVGFEGaussianBlurElementBinding.h"
#include "SVGFEImageElementBinding.h"
#include "SVGFEMergeElementBinding.h"
#include "SVGFEMergeNodeElementBinding.h"
#include "SVGFEMorphologyElementBinding.h"
#include "SVGFEOffsetElementBinding.h"
#include "SVGFEPointLightElementBinding.h"
#include "SVGFESpecularLightingElementBinding.h"
#include "SVGFESpotLightElementBinding.h"
#include "SVGFETileElementBinding.h"
#include "SVGFETurbulenceElementBinding.h"
#include "SVGFilterElementBinding.h"
#include "SVGForeignObjectElementBinding.h"
#include "SVGGElementBinding.h"
#include "SVGGeometryElementBinding.h"
#include "SVGGradientElementBinding.h"
#include "SVGGraphicsElementBinding.h"
#include "SVGImageElementBinding.h"
#include "SVGLengthBinding.h"
#include "SVGLengthListBinding.h"
#include "SVGLineElementBinding.h"
#include "SVGLinearGradientElementBinding.h"
#include "SVGMPathElementBinding.h"
#include "SVGMarkerElementBinding.h"
#include "SVGMaskElementBinding.h"
#include "SVGMatrixBinding.h"
#include "SVGMetadataElementBinding.h"
#include "SVGNumberBinding.h"
#include "SVGNumberListBinding.h"
#include "SVGPathElementBinding.h"
#include "SVGPathSegListBinding.h"
#include "SVGPatternElementBinding.h"
#include "SVGPointBinding.h"
#include "SVGPointListBinding.h"
#include "SVGPolygonElementBinding.h"
#include "SVGPolylineElementBinding.h"
#include "SVGPreserveAspectRatioBinding.h"
#include "SVGRadialGradientElementBinding.h"
#include "SVGRectBinding.h"
#include "SVGRectElementBinding.h"
#include "SVGSVGElementBinding.h"
#include "SVGScriptElementBinding.h"
#include "SVGSetElementBinding.h"
#include "SVGStopElementBinding.h"
#include "SVGStringListBinding.h"
#include "SVGStyleElementBinding.h"
#include "SVGSwitchElementBinding.h"
#include "SVGSymbolElementBinding.h"
#include "SVGTSpanElementBinding.h"
#include "SVGTextContentElementBinding.h"
#include "SVGTextElementBinding.h"
#include "SVGTextPathElementBinding.h"
#include "SVGTextPositioningElementBinding.h"
#include "SVGTitleElementBinding.h"
#include "SVGTransformBinding.h"
#include "SVGTransformListBinding.h"
#include "SVGUnitTypesBinding.h"
#include "SVGUseElementBinding.h"
#include "SVGViewElementBinding.h"
#include "SanitizerBinding.h"
#include "ScreenBinding.h"
#include "ScreenOrientationBinding.h"
#include "ScriptProcessorNodeBinding.h"
#include "ScrollAreaEventBinding.h"
#include "ScrollViewChangeEventBinding.h"
#include "SecurityPolicyViolationEventBinding.h"
#include "SelectionBinding.h"
#include "ServiceWorkerBinding.h"
#include "ServiceWorkerContainerBinding.h"
#include "ServiceWorkerRegistrationBinding.h"
#include "SessionStoreUtilsBinding.h"
#include "ShadowRootBinding.h"
#include "SharedWorkerBinding.h"
#include "SimpleGestureEventBinding.h"
#include "SourceBufferBinding.h"
#include "SourceBufferListBinding.h"
#include "SpeechGrammarBinding.h"
#include "SpeechGrammarListBinding.h"
#include "SpeechRecognitionAlternativeBinding.h"
#include "SpeechRecognitionBinding.h"
#include "SpeechRecognitionErrorBinding.h"
#include "SpeechRecognitionEventBinding.h"
#include "SpeechRecognitionResultBinding.h"
#include "SpeechRecognitionResultListBinding.h"
#include "SpeechSynthesisBinding.h"
#include "SpeechSynthesisErrorEventBinding.h"
#include "SpeechSynthesisEventBinding.h"
#include "SpeechSynthesisUtteranceBinding.h"
#include "SpeechSynthesisVoiceBinding.h"
#include "StaticRangeBinding.h"
#include "StereoPannerNodeBinding.h"
#include "StorageBinding.h"
#include "StorageEventBinding.h"
#include "StorageManagerBinding.h"
#include "StreamFilterBinding.h"
#include "StreamFilterDataEventBinding.h"
#include "StructuredCloneHolderBinding.h"
#include "StructuredCloneTesterBinding.h"
#include "StyleSheetApplicableStateChangeEventBinding.h"
#include "StyleSheetBinding.h"
#include "StyleSheetListBinding.h"
#include "SubmitEventBinding.h"
#include "SubtleCryptoBinding.h"
#include "TCPServerSocketBinding.h"
#include "TCPServerSocketEventBinding.h"
#include "TCPSocketBinding.h"
#include "TCPSocketErrorEventBinding.h"
#include "TCPSocketEventBinding.h"
#include "TelemetryStopwatchBinding.h"
#include "TextBinding.h"
#include "TextClauseBinding.h"
#include "TextDecoderBinding.h"
#include "TextEncoderBinding.h"
#include "TextTrackBinding.h"
#include "TextTrackCueBinding.h"
#include "TextTrackCueListBinding.h"
#include "TextTrackListBinding.h"
#include "TimeEventBinding.h"
#include "TimeRangesBinding.h"
#include "TouchBinding.h"
#include "TouchEventBinding.h"
#include "TouchListBinding.h"
#include "TrackEventBinding.h"
#include "TransceiverImplBinding.h"
#include "TransitionEventBinding.h"
#include "TreeColumnBinding.h"
#include "TreeColumnsBinding.h"
#include "TreeContentViewBinding.h"
#include "TreeWalkerBinding.h"
#include "U2FBinding.h"
#include "UDPMessageEventBinding.h"
#include "UDPSocketBinding.h"
#include "UIEventBinding.h"
#include "URLBinding.h"
#include "URLSearchParamsBinding.h"
#include "UserInteractionBinding.h"
#include "UserProximityEventBinding.h"
#include "VRDisplayBinding.h"
#include "VRDisplayEventBinding.h"
#include "VRServiceTestBinding.h"
#include "VTTCueBinding.h"
#include "VTTRegionBinding.h"
#include "ValidityStateBinding.h"
#include "VideoPlaybackQualityBinding.h"
#include "VideoTrackBinding.h"
#include "VideoTrackListBinding.h"
#include "VisualViewportBinding.h"
#include "WaveShaperNodeBinding.h"
#include "WebAuthenticationBinding.h"
#include "WebExtensionContentScriptBinding.h"
#include "WebExtensionPolicyBinding.h"
#include "WebGL2RenderingContextBinding.h"
#include "WebGLContextEventBinding.h"
#include "WebGLRenderingContextBinding.h"
#include "WebGPUBinding.h"
#include "WebSocketBinding.h"
#include "WebXRBinding.h"
#include "WebrtcDeprecatedBinding.h"
#include "WebrtcGlobalInformationBinding.h"
#include "WheelEventBinding.h"
#include "WindowBinding.h"
#include "WindowGlobalActorsBinding.h"
#include "WindowRootBinding.h"
#include "WorkerBinding.h"
#include "WorkletBinding.h"
#include "XMLDocumentBinding.h"
#include "XMLHttpRequestBinding.h"
#include "XMLHttpRequestEventTargetBinding.h"
#include "XMLHttpRequestUploadBinding.h"
#include "XMLSerializerBinding.h"
#include "XPathEvaluatorBinding.h"
#include "XPathExpressionBinding.h"
#include "XPathResultBinding.h"
#include "XRInputSourceEventBinding.h"
#include "XRInputSourcesChangeEventBinding.h"
#include "XRReferenceSpaceEventBinding.h"
#include "XRSessionEventBinding.h"
#include "XSLTProcessorBinding.h"
#include "XULCommandEventBinding.h"
#include "XULElementBinding.h"
#include "XULFrameElementBinding.h"
#include "XULMenuElementBinding.h"
#include "XULPopupElementBinding.h"
#include "XULTextElementBinding.h"
#include "XULTreeElementBinding.h"
#include "js/String.h"
#include "mozilla/PerfectHash.h"
#include "mozilla/dom/PrototypeList.h"
#include "mozilla/dom/WebIDLGlobalNameHash.h"

namespace mozilla {
namespace dom {
const uint32_t WebIDLGlobalNameHash::sCount = 810;

const char WebIDLGlobalNameHash::sNames[] =
  /* 0 */ "APZHitResultFlags\0"
  /* 18 */ "AbortController\0"
  /* 34 */ "AbortSignal\0"
  /* 46 */ "AbstractRange\0"
  /* 60 */ "AccessibleNode\0"
  /* 75 */ "Addon\0"
  /* 81 */ "AddonEvent\0"
  /* 92 */ "AddonInstall\0"
  /* 105 */ "AddonManager\0"
  /* 118 */ "AddonManagerPermissions\0"
  /* 142 */ "AnalyserNode\0"
  /* 155 */ "Animation\0"
  /* 165 */ "AnimationEffect\0"
  /* 181 */ "AnimationEvent\0"
  /* 196 */ "AnimationPlaybackEvent\0"
  /* 219 */ "AnimationTimeline\0"
  /* 237 */ "AnonymousContent\0"
  /* 254 */ "Attr\0"
  /* 259 */ "AudioBuffer\0"
  /* 271 */ "AudioBufferSourceNode\0"
  /* 293 */ "AudioContext\0"
  /* 306 */ "AudioDestinationNode\0"
  /* 327 */ "AudioListener\0"
  /* 341 */ "AudioNode\0"
  /* 351 */ "AudioParam\0"
  /* 362 */ "AudioParamMap\0"
  /* 376 */ "AudioProcessingEvent\0"
  /* 397 */ "AudioScheduledSourceNode\0"
  /* 422 */ "AudioTrack\0"
  /* 433 */ "AudioTrackList\0"
  /* 448 */ "AudioWorklet\0"
  /* 461 */ "AudioWorkletNode\0"
  /* 478 */ "AuthenticatorAssertionResponse\0"
  /* 509 */ "AuthenticatorAttestationResponse\0"
  /* 542 */ "AuthenticatorResponse\0"
  /* 564 */ "BarProp\0"
  /* 572 */ "BaseAudioContext\0"
  /* 589 */ "BatteryManager\0"
  /* 604 */ "BeforeUnloadEvent\0"
  /* 622 */ "BiquadFilterNode\0"
  /* 639 */ "Blob\0"
  /* 644 */ "BlobEvent\0"
  /* 654 */ "BroadcastChannel\0"
  /* 671 */ "BrowsingContext\0"
  /* 687 */ "BrowsingContextGroup\0"
  /* 708 */ "CDATASection\0"
  /* 721 */ "CSS\0"
  /* 725 */ "CSS2Properties\0"
  /* 740 */ "CSSAnimation\0"
  /* 753 */ "CSSConditionRule\0"
  /* 770 */ "CSSCounterStyleRule\0"
  /* 790 */ "CSSFontFaceRule\0"
  /* 806 */ "CSSFontFeatureValuesRule\0"
  /* 831 */ "CSSGroupingRule\0"
  /* 847 */ "CSSImportRule\0"
  /* 861 */ "CSSKeyframeRule\0"
  /* 877 */ "CSSKeyframesRule\0"
  /* 894 */ "CSSMediaRule\0"
  /* 907 */ "CSSMozDocumentRule\0"
  /* 926 */ "CSSNamespaceRule\0"
  /* 943 */ "CSSPageRule\0"
  /* 955 */ "CSSPseudoElement\0"
  /* 972 */ "CSSRule\0"
  /* 980 */ "CSSRuleList\0"
  /* 992 */ "CSSStyleDeclaration\0"
  /* 1012 */ "CSSStyleRule\0"
  /* 1025 */ "CSSStyleSheet\0"
  /* 1039 */ "CSSSupportsRule\0"
  /* 1055 */ "CSSTransition\0"
  /* 1069 */ "Cache\0"
  /* 1075 */ "CacheStorage\0"
  /* 1088 */ "CallbackDebuggerNotification\0"
  /* 1117 */ "CanonicalBrowsingContext\0"
  /* 1142 */ "CanvasCaptureMediaStream\0"
  /* 1167 */ "CanvasGradient\0"
  /* 1182 */ "CanvasPattern\0"
  /* 1196 */ "CanvasRenderingContext2D\0"
  /* 1221 */ "CaretPosition\0"
  /* 1235 */ "CaretStateChangedEvent\0"
  /* 1258 */ "ChannelMergerNode\0"
  /* 1276 */ "ChannelSplitterNode\0"
  /* 1296 */ "ChannelWrapper\0"
  /* 1311 */ "CharacterData\0"
  /* 1325 */ "CheckerboardReportService\0"
  /* 1351 */ "ChildProcessMessageManager\0"
  /* 1378 */ "ChildSHistory\0"
  /* 1392 */ "ChromeMessageBroadcaster\0"
  /* 1417 */ "ChromeMessageSender\0"
  /* 1437 */ "ChromeNodeList\0"
  /* 1452 */ "ChromeUtils\0"
  /* 1464 */ "ChromeWorker\0"
  /* 1477 */ "Clipboard\0"
  /* 1487 */ "ClipboardEvent\0"
  /* 1502 */ "ClipboardItem\0"
  /* 1516 */ "ClonedErrorHolder\0"
  /* 1534 */ "CloseEvent\0"
  /* 1545 */ "CommandEvent\0"
  /* 1558 */ "Comment\0"
  /* 1566 */ "CompositionEvent\0"
  /* 1583 */ "ConsoleInstance\0"
  /* 1599 */ "ConstantSourceNode\0"
  /* 1618 */ "ContentFrameMessageManager\0"
  /* 1645 */ "ContentProcessMessageManager\0"
  /* 1674 */ "ConvolverNode\0"
  /* 1688 */ "CreateOfferRequest\0"
  /* 1707 */ "Credential\0"
  /* 1718 */ "CredentialsContainer\0"
  /* 1739 */ "Crypto\0"
  /* 1746 */ "CryptoKey\0"
  /* 1756 */ "CustomElementRegistry\0"
  /* 1778 */ "CustomEvent\0"
  /* 1790 */ "DOMException\0"
  /* 1803 */ "DOMImplementation\0"
  /* 1821 */ "DOMLocalization\0"
  /* 1837 */ "DOMMatrix\0"
  /* 1847 */ "WebKitCSSMatrix\0"
  /* 1863 */ "DOMMatrixReadOnly\0"
  /* 1881 */ "DOMParser\0"
  /* 1891 */ "DOMPoint\0"
  /* 1900 */ "DOMPointReadOnly\0"
  /* 1917 */ "DOMQuad\0"
  /* 1925 */ "DOMRect\0"
  /* 1933 */ "DOMRectList\0"
  /* 1945 */ "DOMRectReadOnly\0"
  /* 1961 */ "DOMRequest\0"
  /* 1972 */ "DOMStringList\0"
  /* 1986 */ "DOMStringMap\0"
  /* 1999 */ "DOMTokenList\0"
  /* 2012 */ "DataTransfer\0"
  /* 2025 */ "DataTransferItem\0"
  /* 2042 */ "DataTransferItemList\0"
  /* 2063 */ "DebuggerNotification\0"
  /* 2084 */ "DebuggerNotificationObserver\0"
  /* 2113 */ "DelayNode\0"
  /* 2123 */ "DeprecationReportBody\0"
  /* 2145 */ "DeviceLightEvent\0"
  /* 2162 */ "DeviceMotionEvent\0"
  /* 2180 */ "DeviceOrientationEvent\0"
  /* 2203 */ "DeviceProximityEvent\0"
  /* 2224 */ "Directory\0"
  /* 2234 */ "Document\0"
  /* 2243 */ "DocumentFragment\0"
  /* 2260 */ "DocumentTimeline\0"
  /* 2277 */ "DocumentType\0"
  /* 2290 */ "DominatorTree\0"
  /* 2304 */ "DragEvent\0"
  /* 2314 */ "DynamicsCompressorNode\0"
  /* 2337 */ "Element\0"
  /* 2345 */ "ElementInternals\0"
  /* 2362 */ "ErrorEvent\0"
  /* 2373 */ "Event\0"
  /* 2379 */ "EventCallbackDebuggerNotification\0"
  /* 2413 */ "EventCounts\0"
  /* 2425 */ "EventSource\0"
  /* 2437 */ "EventTarget\0"
  /* 2449 */ "FeaturePolicyViolationReportBody\0"
  /* 2482 */ "FetchObserver\0"
  /* 2496 */ "File\0"
  /* 2501 */ "FileList\0"
  /* 2510 */ "FileReader\0"
  /* 2521 */ "FileSystem\0"
  /* 2532 */ "FileSystemDirectoryEntry\0"
  /* 2557 */ "FileSystemDirectoryReader\0"
  /* 2583 */ "FileSystemEntry\0"
  /* 2599 */ "FileSystemFileEntry\0"
  /* 2619 */ "Flex\0"
  /* 2624 */ "FlexItemValues\0"
  /* 2639 */ "FlexLineValues\0"
  /* 2654 */ "FluentBundle\0"
  /* 2667 */ "FluentPattern\0"
  /* 2681 */ "FluentResource\0"
  /* 2696 */ "FocusEvent\0"
  /* 2707 */ "FontFace\0"
  /* 2716 */ "FontFaceSet\0"
  /* 2728 */ "FontFaceSetLoadEvent\0"
  /* 2749 */ "FormData\0"
  /* 2758 */ "FormDataEvent\0"
  /* 2772 */ "FrameCrashedEvent\0"
  /* 2790 */ "FrameLoader\0"
  /* 2802 */ "GPU\0"
  /* 2806 */ "GPUAdapter\0"
  /* 2817 */ "GPUBindGroup\0"
  /* 2830 */ "GPUBindGroupLayout\0"
  /* 2849 */ "GPUBuffer\0"
  /* 2859 */ "GPUBufferUsage\0"
  /* 2874 */ "GPUCanvasContext\0"
  /* 2891 */ "GPUColorWrite\0"
  /* 2905 */ "GPUCommandBuffer\0"
  /* 2922 */ "GPUCommandEncoder\0"
  /* 2940 */ "GPUComputePassEncoder\0"
  /* 2962 */ "GPUComputePipeline\0"
  /* 2981 */ "GPUDevice\0"
  /* 2991 */ "GPUDeviceLostInfo\0"
  /* 3009 */ "GPUFence\0"
  /* 3018 */ "GPUMapMode\0"
  /* 3029 */ "GPUOutOfMemoryError\0"
  /* 3049 */ "GPUPipelineLayout\0"
  /* 3067 */ "GPUQueue\0"
  /* 3076 */ "GPURenderBundle\0"
  /* 3092 */ "GPURenderBundleEncoder\0"
  /* 3115 */ "GPURenderPassEncoder\0"
  /* 3136 */ "GPURenderPipeline\0"
  /* 3154 */ "GPUSampler\0"
  /* 3165 */ "GPUShaderModule\0"
  /* 3181 */ "GPUShaderStage\0"
  /* 3196 */ "GPUSwapChain\0"
  /* 3209 */ "GPUTexture\0"
  /* 3220 */ "GPUTextureUsage\0"
  /* 3236 */ "GPUTextureView\0"
  /* 3251 */ "GPUUncapturedErrorEvent\0"
  /* 3275 */ "GPUValidationError\0"
  /* 3294 */ "GainNode\0"
  /* 3303 */ "Gamepad\0"
  /* 3311 */ "GamepadAxisMoveEvent\0"
  /* 3332 */ "GamepadButton\0"
  /* 3346 */ "GamepadButtonEvent\0"
  /* 3365 */ "GamepadEvent\0"
  /* 3378 */ "GamepadHapticActuator\0"
  /* 3400 */ "GamepadLightIndicator\0"
  /* 3422 */ "GamepadPose\0"
  /* 3434 */ "GamepadServiceTest\0"
  /* 3453 */ "GamepadTouch\0"
  /* 3466 */ "Geolocation\0"
  /* 3478 */ "GeolocationCoordinates\0"
  /* 3501 */ "GeolocationPosition\0"
  /* 3521 */ "GeolocationPositionError\0"
  /* 3546 */ "GleanCategory\0"
  /* 3560 */ "GleanImpl\0"
  /* 3570 */ "GleanLabeled\0"
  /* 3583 */ "GleanPingsImpl\0"
  /* 3598 */ "Grid\0"
  /* 3603 */ "GridArea\0"
  /* 3612 */ "GridDimension\0"
  /* 3626 */ "GridLine\0"
  /* 3635 */ "GridLines\0"
  /* 3645 */ "GridTrack\0"
  /* 3655 */ "GridTracks\0"
  /* 3666 */ "HTMLAllCollection\0"
  /* 3684 */ "HTMLAnchorElement\0"
  /* 3702 */ "HTMLAreaElement\0"
  /* 3718 */ "HTMLAudioElement\0"
  /* 3735 */ "Audio\0"
  /* 3741 */ "HTMLBRElement\0"
  /* 3755 */ "HTMLBaseElement\0"
  /* 3771 */ "HTMLBodyElement\0"
  /* 3787 */ "HTMLButtonElement\0"
  /* 3805 */ "HTMLCanvasElement\0"
  /* 3823 */ "HTMLCollection\0"
  /* 3838 */ "HTMLDListElement\0"
  /* 3855 */ "HTMLDataElement\0"
  /* 3871 */ "HTMLDataListElement\0"
  /* 3891 */ "HTMLDetailsElement\0"
  /* 3910 */ "HTMLDialogElement\0"
  /* 3928 */ "HTMLDirectoryElement\0"
  /* 3949 */ "HTMLDivElement\0"
  /* 3964 */ "HTMLDocument\0"
  /* 3977 */ "HTMLElement\0"
  /* 3989 */ "HTMLEmbedElement\0"
  /* 4006 */ "HTMLFieldSetElement\0"
  /* 4026 */ "HTMLFontElement\0"
  /* 4042 */ "HTMLFormControlsCollection\0"
  /* 4069 */ "HTMLFormElement\0"
  /* 4085 */ "HTMLFrameElement\0"
  /* 4102 */ "HTMLFrameSetElement\0"
  /* 4122 */ "HTMLHRElement\0"
  /* 4136 */ "HTMLHeadElement\0"
  /* 4152 */ "HTMLHeadingElement\0"
  /* 4171 */ "HTMLHtmlElement\0"
  /* 4187 */ "HTMLIFrameElement\0"
  /* 4205 */ "HTMLImageElement\0"
  /* 4222 */ "Image\0"
  /* 4228 */ "HTMLInputElement\0"
  /* 4245 */ "HTMLLIElement\0"
  /* 4259 */ "HTMLLabelElement\0"
  /* 4276 */ "HTMLLegendElement\0"
  /* 4294 */ "HTMLLinkElement\0"
  /* 4310 */ "HTMLMapElement\0"
  /* 4325 */ "HTMLMarqueeElement\0"
  /* 4344 */ "HTMLMediaElement\0"
  /* 4361 */ "HTMLMenuElement\0"
  /* 4377 */ "HTMLMenuItemElement\0"
  /* 4397 */ "HTMLMetaElement\0"
  /* 4413 */ "HTMLMeterElement\0"
  /* 4430 */ "HTMLModElement\0"
  /* 4445 */ "HTMLOListElement\0"
  /* 4462 */ "HTMLObjectElement\0"
  /* 4480 */ "HTMLOptGroupElement\0"
  /* 4500 */ "HTMLOptionElement\0"
  /* 4518 */ "Option\0"
  /* 4525 */ "HTMLOptionsCollection\0"
  /* 4547 */ "HTMLOutputElement\0"
  /* 4565 */ "HTMLParagraphElement\0"
  /* 4586 */ "HTMLParamElement\0"
  /* 4603 */ "HTMLPictureElement\0"
  /* 4622 */ "HTMLPreElement\0"
  /* 4637 */ "HTMLProgressElement\0"
  /* 4657 */ "HTMLQuoteElement\0"
  /* 4674 */ "HTMLScriptElement\0"
  /* 4692 */ "HTMLSelectElement\0"
  /* 4710 */ "HTMLSlotElement\0"
  /* 4726 */ "HTMLSourceElement\0"
  /* 4744 */ "HTMLSpanElement\0"
  /* 4760 */ "HTMLStyleElement\0"
  /* 4777 */ "HTMLTableCaptionElement\0"
  /* 4801 */ "HTMLTableCellElement\0"
  /* 4822 */ "HTMLTableColElement\0"
  /* 4842 */ "HTMLTableElement\0"
  /* 4859 */ "HTMLTableRowElement\0"
  /* 4879 */ "HTMLTableSectionElement\0"
  /* 4903 */ "HTMLTemplateElement\0"
  /* 4923 */ "HTMLTextAreaElement\0"
  /* 4943 */ "HTMLTimeElement\0"
  /* 4959 */ "HTMLTitleElement\0"
  /* 4976 */ "HTMLTrackElement\0"
  /* 4993 */ "HTMLUListElement\0"
  /* 5010 */ "HTMLUnknownElement\0"
  /* 5029 */ "HTMLVideoElement\0"
  /* 5046 */ "HashChangeEvent\0"
  /* 5062 */ "Headers\0"
  /* 5070 */ "HeapSnapshot\0"
  /* 5083 */ "HiddenPluginEvent\0"
  /* 5101 */ "History\0"
  /* 5109 */ "IDBCursor\0"
  /* 5119 */ "IDBCursorWithValue\0"
  /* 5138 */ "IDBDatabase\0"
  /* 5150 */ "IDBFactory\0"
  /* 5161 */ "IDBFileHandle\0"
  /* 5175 */ "IDBFileRequest\0"
  /* 5190 */ "IDBIndex\0"
  /* 5199 */ "IDBKeyRange\0"
  /* 5211 */ "IDBLocaleAwareKeyRange\0"
  /* 5234 */ "IDBMutableFile\0"
  /* 5249 */ "IDBObjectStore\0"
  /* 5264 */ "IDBOpenDBRequest\0"
  /* 5281 */ "IDBRequest\0"
  /* 5292 */ "IDBTransaction\0"
  /* 5307 */ "IDBVersionChangeEvent\0"
  /* 5329 */ "IIRFilterNode\0"
  /* 5343 */ "IOUtils\0"
  /* 5351 */ "IdleDeadline\0"
  /* 5364 */ "ImageBitmap\0"
  /* 5376 */ "ImageBitmapRenderingContext\0"
  /* 5404 */ "ImageCapture\0"
  /* 5417 */ "ImageCaptureErrorEvent\0"
  /* 5440 */ "ImageData\0"
  /* 5450 */ "ImageDocument\0"
  /* 5464 */ "InputEvent\0"
  /* 5475 */ "InspectorFontFace\0"
  /* 5493 */ "InspectorUtils\0"
  /* 5508 */ "InstallTriggerImpl\0"
  /* 5527 */ "IntersectionObserver\0"
  /* 5548 */ "IntersectionObserverEntry\0"
  /* 5574 */ "JSProcessActorChild\0"
  /* 5594 */ "JSProcessActorParent\0"
  /* 5615 */ "JSWindowActorChild\0"
  /* 5634 */ "JSWindowActorParent\0"
  /* 5654 */ "KeyEvent\0"
  /* 5663 */ "KeyboardEvent\0"
  /* 5677 */ "KeyframeEffect\0"
  /* 5692 */ "L10nOverlays\0"
  /* 5705 */ "Localization\0"
  /* 5718 */ "Location\0"
  /* 5727 */ "MIDIAccess\0"
  /* 5738 */ "MIDIConnectionEvent\0"
  /* 5758 */ "MIDIInput\0"
  /* 5768 */ "MIDIInputMap\0"
  /* 5781 */ "MIDIMessageEvent\0"
  /* 5798 */ "MIDIOutput\0"
  /* 5809 */ "MIDIOutputMap\0"
  /* 5823 */ "MIDIPort\0"
  /* 5832 */ "MatchGlob\0"
  /* 5842 */ "MatchPattern\0"
  /* 5855 */ "MatchPatternSet\0"
  /* 5871 */ "MathMLElement\0"
  /* 5885 */ "MediaCapabilities\0"
  /* 5903 */ "MediaCapabilitiesInfo\0"
  /* 5925 */ "MediaControlService\0"
  /* 5945 */ "MediaController\0"
  /* 5961 */ "MediaDeviceInfo\0"
  /* 5977 */ "MediaDevices\0"
  /* 5990 */ "MediaElementAudioSourceNode\0"
  /* 6018 */ "MediaEncryptedEvent\0"
  /* 6038 */ "MediaError\0"
  /* 6049 */ "MediaKeyError\0"
  /* 6063 */ "MediaKeyMessageEvent\0"
  /* 6084 */ "MediaKeySession\0"
  /* 6100 */ "MediaKeyStatusMap\0"
  /* 6118 */ "MediaKeySystemAccess\0"
  /* 6139 */ "MediaKeys\0"
  /* 6149 */ "MediaList\0"
  /* 6159 */ "MediaMetadata\0"
  /* 6173 */ "MediaQueryList\0"
  /* 6188 */ "MediaQueryListEvent\0"
  /* 6208 */ "MediaRecorder\0"
  /* 6222 */ "MediaRecorderErrorEvent\0"
  /* 6246 */ "MediaSession\0"
  /* 6259 */ "MediaSource\0"
  /* 6271 */ "MediaStream\0"
  /* 6283 */ "MediaStreamAudioDestinationNode\0"
  /* 6315 */ "MediaStreamAudioSourceNode\0"
  /* 6342 */ "MediaStreamEvent\0"
  /* 6359 */ "MediaStreamTrack\0"
  /* 6376 */ "MediaStreamTrackAudioSourceNode\0"
  /* 6408 */ "MediaStreamTrackEvent\0"
  /* 6430 */ "MerchantValidationEvent\0"
  /* 6454 */ "MessageBroadcaster\0"
  /* 6473 */ "MessageChannel\0"
  /* 6488 */ "MessageEvent\0"
  /* 6501 */ "MessageListenerManager\0"
  /* 6524 */ "MessagePort\0"
  /* 6536 */ "MessageSender\0"
  /* 6550 */ "MimeType\0"
  /* 6559 */ "MimeTypeArray\0"
  /* 6573 */ "MouseEvent\0"
  /* 6584 */ "MouseScrollEvent\0"
  /* 6601 */ "MozCanvasPrintState\0"
  /* 6621 */ "MozDocumentMatcher\0"
  /* 6640 */ "MozDocumentObserver\0"
  /* 6660 */ "MozQueryInterface\0"
  /* 6678 */ "MozSharedMap\0"
  /* 6691 */ "MozSharedMapChangeEvent\0"
  /* 6715 */ "MozStorageAsyncStatementParams\0"
  /* 6746 */ "MozStorageStatementParams\0"
  /* 6772 */ "MozStorageStatementRow\0"
  /* 6795 */ "MozWritableSharedMap\0"
  /* 6816 */ "MutationEvent\0"
  /* 6830 */ "MutationObserver\0"
  /* 6847 */ "MutationRecord\0"
  /* 6862 */ "NamedNodeMap\0"
  /* 6875 */ "Navigator\0"
  /* 6885 */ "NetworkInformation\0"
  /* 6904 */ "Node\0"
  /* 6909 */ "NodeFilter\0"
  /* 6920 */ "NodeIterator\0"
  /* 6933 */ "NodeList\0"
  /* 6942 */ "Notification\0"
  /* 6955 */ "NotifyPaintEvent\0"
  /* 6972 */ "OfflineAudioCompletionEvent\0"
  /* 7000 */ "OfflineAudioContext\0"
  /* 7020 */ "OfflineResourceList\0"
  /* 7040 */ "OffscreenCanvas\0"
  /* 7056 */ "OscillatorNode\0"
  /* 7071 */ "PageTransitionEvent\0"
  /* 7091 */ "PaintRequest\0"
  /* 7104 */ "PaintRequestList\0"
  /* 7121 */ "PannerNode\0"
  /* 7132 */ "ParentProcessMessageManager\0"
  /* 7160 */ "Path2D\0"
  /* 7167 */ "PathUtils\0"
  /* 7177 */ "PaymentAddress\0"
  /* 7192 */ "PaymentMethodChangeEvent\0"
  /* 7217 */ "PaymentRequest\0"
  /* 7232 */ "PaymentRequestUpdateEvent\0"
  /* 7258 */ "PaymentResponse\0"
  /* 7274 */ "PeerConnectionImpl\0"
  /* 7293 */ "PeerConnectionObserver\0"
  /* 7316 */ "Performance\0"
  /* 7328 */ "PerformanceEntry\0"
  /* 7345 */ "PerformanceEntryEvent\0"
  /* 7367 */ "PerformanceEventTiming\0"
  /* 7390 */ "PerformanceMark\0"
  /* 7406 */ "PerformanceMeasure\0"
  /* 7425 */ "PerformanceNavigation\0"
  /* 7447 */ "PerformanceNavigationTiming\0"
  /* 7475 */ "PerformanceObserver\0"
  /* 7495 */ "PerformanceObserverEntryList\0"
  /* 7524 */ "PerformancePaintTiming\0"
  /* 7547 */ "PerformanceResourceTiming\0"
  /* 7573 */ "PerformanceServerTiming\0"
  /* 7597 */ "PerformanceTiming\0"
  /* 7615 */ "PeriodicWave\0"
  /* 7628 */ "PermissionStatus\0"
  /* 7645 */ "Permissions\0"
  /* 7657 */ "PlacesBookmark\0"
  /* 7672 */ "PlacesBookmarkAddition\0"
  /* 7695 */ "PlacesBookmarkRemoved\0"
  /* 7717 */ "PlacesEvent\0"
  /* 7729 */ "PlacesFavicon\0"
  /* 7743 */ "PlacesHistoryCleared\0"
  /* 7764 */ "PlacesObservers\0"
  /* 7780 */ "PlacesRanking\0"
  /* 7794 */ "PlacesVisit\0"
  /* 7806 */ "PlacesVisitRemoved\0"
  /* 7825 */ "PlacesVisitTitle\0"
  /* 7842 */ "PlacesWeakCallbackWrapper\0"
  /* 7868 */ "Plugin\0"
  /* 7875 */ "PluginArray\0"
  /* 7887 */ "PluginCrashedEvent\0"
  /* 7906 */ "PointerEvent\0"
  /* 7919 */ "PopStateEvent\0"
  /* 7933 */ "PopupBlockedEvent\0"
  /* 7951 */ "PopupPositionedEvent\0"
  /* 7972 */ "PositionStateEvent\0"
  /* 7991 */ "PrecompiledScript\0"
  /* 8009 */ "Presentation\0"
  /* 8022 */ "PresentationAvailability\0"
  /* 8047 */ "PresentationConnection\0"
  /* 8070 */ "PresentationConnectionAvailableEvent\0"
  /* 8107 */ "PresentationConnectionCloseEvent\0"
  /* 8140 */ "PresentationConnectionList\0"
  /* 8167 */ "PresentationReceiver\0"
  /* 8188 */ "PresentationRequest\0"
  /* 8208 */ "PrioEncoder\0"
  /* 8220 */ "ProcessMessageManager\0"
  /* 8242 */ "ProcessingInstruction\0"
  /* 8264 */ "ProgressEvent\0"
  /* 8278 */ "PromiseDebugging\0"
  /* 8295 */ "PromiseRejectionEvent\0"
  /* 8317 */ "PublicKeyCredential\0"
  /* 8337 */ "PushManager\0"
  /* 8349 */ "PushManagerImpl\0"
  /* 8365 */ "PushSubscription\0"
  /* 8382 */ "PushSubscriptionOptions\0"
  /* 8406 */ "RTCCertificate\0"
  /* 8421 */ "RTCDTMFSender\0"
  /* 8435 */ "RTCDTMFToneChangeEvent\0"
  /* 8458 */ "RTCDataChannel\0"
  /* 8473 */ "RTCDataChannelEvent\0"
  /* 8493 */ "RTCDtlsTransport\0"
  /* 8510 */ "RTCIceCandidate\0"
  /* 8526 */ "RTCPeerConnection\0"
  /* 8544 */ "RTCPeerConnectionIceEvent\0"
  /* 8570 */ "RTCPeerConnectionStatic\0"
  /* 8594 */ "RTCRtpReceiver\0"
  /* 8609 */ "RTCRtpSender\0"
  /* 8622 */ "RTCRtpTransceiver\0"
  /* 8640 */ "RTCSessionDescription\0"
  /* 8662 */ "RTCStatsReport\0"
  /* 8677 */ "RTCTrackEvent\0"
  /* 8691 */ "RadioNodeList\0"
  /* 8705 */ "Range\0"
  /* 8711 */ "Report\0"
  /* 8718 */ "ReportBody\0"
  /* 8729 */ "ReportingObserver\0"
  /* 8747 */ "Request\0"
  /* 8755 */ "ResizeObserver\0"
  /* 8770 */ "ResizeObserverEntry\0"
  /* 8790 */ "ResizeObserverSize\0"
  /* 8809 */ "Response\0"
  /* 8818 */ "SVGAElement\0"
  /* 8830 */ "SVGAngle\0"
  /* 8839 */ "SVGAnimateElement\0"
  /* 8857 */ "SVGAnimateMotionElement\0"
  /* 8881 */ "SVGAnimateTransformElement\0"
  /* 8908 */ "SVGAnimatedAngle\0"
  /* 8925 */ "SVGAnimatedBoolean\0"
  /* 8944 */ "SVGAnimatedEnumeration\0"
  /* 8967 */ "SVGAnimatedInteger\0"
  /* 8986 */ "SVGAnimatedLength\0"
  /* 9004 */ "SVGAnimatedLengthList\0"
  /* 9026 */ "SVGAnimatedNumber\0"
  /* 9044 */ "SVGAnimatedNumberList\0"
  /* 9066 */ "SVGAnimatedPreserveAspectRatio\0"
  /* 9097 */ "SVGAnimatedRect\0"
  /* 9113 */ "SVGAnimatedString\0"
  /* 9131 */ "SVGAnimatedTransformList\0"
  /* 9156 */ "SVGAnimationElement\0"
  /* 9176 */ "SVGCircleElement\0"
  /* 9193 */ "SVGClipPathElement\0"
  /* 9212 */ "SVGComponentTransferFunctionElement\0"
  /* 9248 */ "SVGDefsElement\0"
  /* 9263 */ "SVGDescElement\0"
  /* 9278 */ "SVGElement\0"
  /* 9289 */ "SVGEllipseElement\0"
  /* 9307 */ "SVGFEBlendElement\0"
  /* 9325 */ "SVGFEColorMatrixElement\0"
  /* 9349 */ "SVGFEComponentTransferElement\0"
  /* 9379 */ "SVGFECompositeElement\0"
  /* 9401 */ "SVGFEConvolveMatrixElement\0"
  /* 9428 */ "SVGFEDiffuseLightingElement\0"
  /* 9456 */ "SVGFEDisplacementMapElement\0"
  /* 9484 */ "SVGFEDistantLightElement\0"
  /* 9509 */ "SVGFEDropShadowElement\0"
  /* 9532 */ "SVGFEFloodElement\0"
  /* 9550 */ "SVGFEFuncAElement\0"
  /* 9568 */ "SVGFEFuncBElement\0"
  /* 9586 */ "SVGFEFuncGElement\0"
  /* 9604 */ "SVGFEFuncRElement\0"
  /* 9622 */ "SVGFEGaussianBlurElement\0"
  /* 9647 */ "SVGFEImageElement\0"
  /* 9665 */ "SVGFEMergeElement\0"
  /* 9683 */ "SVGFEMergeNodeElement\0"
  /* 9705 */ "SVGFEMorphologyElement\0"
  /* 9728 */ "SVGFEOffsetElement\0"
  /* 9747 */ "SVGFEPointLightElement\0"
  /* 9770 */ "SVGFESpecularLightingElement\0"
  /* 9799 */ "SVGFESpotLightElement\0"
  /* 9821 */ "SVGFETileElement\0"
  /* 9838 */ "SVGFETurbulenceElement\0"
  /* 9861 */ "SVGFilterElement\0"
  /* 9878 */ "SVGForeignObjectElement\0"
  /* 9902 */ "SVGGElement\0"
  /* 9914 */ "SVGGeometryElement\0"
  /* 9933 */ "SVGGradientElement\0"
  /* 9952 */ "SVGGraphicsElement\0"
  /* 9971 */ "SVGImageElement\0"
  /* 9987 */ "SVGLength\0"
  /* 9997 */ "SVGLengthList\0"
  /* 10011 */ "SVGLineElement\0"
  /* 10026 */ "SVGLinearGradientElement\0"
  /* 10051 */ "SVGMPathElement\0"
  /* 10067 */ "SVGMarkerElement\0"
  /* 10084 */ "SVGMaskElement\0"
  /* 10099 */ "SVGMatrix\0"
  /* 10109 */ "SVGMetadataElement\0"
  /* 10128 */ "SVGNumber\0"
  /* 10138 */ "SVGNumberList\0"
  /* 10152 */ "SVGPathElement\0"
  /* 10167 */ "SVGPathSegList\0"
  /* 10182 */ "SVGPatternElement\0"
  /* 10200 */ "SVGPoint\0"
  /* 10209 */ "SVGPointList\0"
  /* 10222 */ "SVGPolygonElement\0"
  /* 10240 */ "SVGPolylineElement\0"
  /* 10259 */ "SVGPreserveAspectRatio\0"
  /* 10282 */ "SVGRadialGradientElement\0"
  /* 10307 */ "SVGRect\0"
  /* 10315 */ "SVGRectElement\0"
  /* 10330 */ "SVGSVGElement\0"
  /* 10344 */ "SVGScriptElement\0"
  /* 10361 */ "SVGSetElement\0"
  /* 10375 */ "SVGStopElement\0"
  /* 10390 */ "SVGStringList\0"
  /* 10404 */ "SVGStyleElement\0"
  /* 10420 */ "SVGSwitchElement\0"
  /* 10437 */ "SVGSymbolElement\0"
  /* 10454 */ "SVGTSpanElement\0"
  /* 10470 */ "SVGTextContentElement\0"
  /* 10492 */ "SVGTextElement\0"
  /* 10507 */ "SVGTextPathElement\0"
  /* 10526 */ "SVGTextPositioningElement\0"
  /* 10552 */ "SVGTitleElement\0"
  /* 10568 */ "SVGTransform\0"
  /* 10581 */ "SVGTransformList\0"
  /* 10598 */ "SVGUnitTypes\0"
  /* 10611 */ "SVGUseElement\0"
  /* 10625 */ "SVGViewElement\0"
  /* 10640 */ "Sanitizer\0"
  /* 10650 */ "Screen\0"
  /* 10657 */ "ScreenLuminance\0"
  /* 10673 */ "ScreenOrientation\0"
  /* 10691 */ "ScriptProcessorNode\0"
  /* 10711 */ "ScrollAreaEvent\0"
  /* 10727 */ "ScrollViewChangeEvent\0"
  /* 10749 */ "SecurityPolicyViolationEvent\0"
  /* 10778 */ "Selection\0"
  /* 10788 */ "ServiceWorker\0"
  /* 10802 */ "ServiceWorkerContainer\0"
  /* 10825 */ "ServiceWorkerRegistration\0"
  /* 10851 */ "SessionStoreUtils\0"
  /* 10869 */ "ShadowRoot\0"
  /* 10880 */ "SharedWorker\0"
  /* 10893 */ "SimpleGestureEvent\0"
  /* 10912 */ "SourceBuffer\0"
  /* 10925 */ "SourceBufferList\0"
  /* 10942 */ "SpeechGrammar\0"
  /* 10956 */ "webkitSpeechGrammar\0"
  /* 10976 */ "SpeechGrammarList\0"
  /* 10994 */ "webkitSpeechGrammarList\0"
  /* 11018 */ "SpeechRecognition\0"
  /* 11036 */ "webkitSpeechRecognition\0"
  /* 11060 */ "SpeechRecognitionAlternative\0"
  /* 11089 */ "SpeechRecognitionError\0"
  /* 11112 */ "SpeechRecognitionEvent\0"
  /* 11135 */ "SpeechRecognitionResult\0"
  /* 11159 */ "SpeechRecognitionResultList\0"
  /* 11187 */ "SpeechSynthesis\0"
  /* 11203 */ "SpeechSynthesisErrorEvent\0"
  /* 11229 */ "SpeechSynthesisEvent\0"
  /* 11250 */ "SpeechSynthesisUtterance\0"
  /* 11275 */ "SpeechSynthesisVoice\0"
  /* 11296 */ "StaticRange\0"
  /* 11308 */ "StereoPannerNode\0"
  /* 11325 */ "Storage\0"
  /* 11333 */ "StorageEvent\0"
  /* 11346 */ "StorageManager\0"
  /* 11361 */ "StreamFilter\0"
  /* 11374 */ "StreamFilterDataEvent\0"
  /* 11396 */ "StructuredCloneHolder\0"
  /* 11418 */ "StructuredCloneTester\0"
  /* 11440 */ "StyleSheet\0"
  /* 11451 */ "StyleSheetApplicableStateChangeEvent\0"
  /* 11488 */ "StyleSheetList\0"
  /* 11503 */ "SubmitEvent\0"
  /* 11515 */ "SubtleCrypto\0"
  /* 11528 */ "SyncMessageSender\0"
  /* 11546 */ "TCPServerSocket\0"
  /* 11562 */ "TCPServerSocketEvent\0"
  /* 11583 */ "TCPSocket\0"
  /* 11593 */ "TCPSocketErrorEvent\0"
  /* 11613 */ "TCPSocketEvent\0"
  /* 11628 */ "TelemetryStopwatch\0"
  /* 11647 */ "TestingDeprecatedInterface\0"
  /* 11674 */ "Text\0"
  /* 11679 */ "TextClause\0"
  /* 11690 */ "TextDecoder\0"
  /* 11702 */ "TextEncoder\0"
  /* 11714 */ "TextMetrics\0"
  /* 11726 */ "TextTrack\0"
  /* 11736 */ "TextTrackCue\0"
  /* 11749 */ "TextTrackCueList\0"
  /* 11766 */ "TextTrackList\0"
  /* 11780 */ "TimeEvent\0"
  /* 11790 */ "TimeRanges\0"
  /* 11801 */ "Touch\0"
  /* 11807 */ "TouchEvent\0"
  /* 11818 */ "TouchList\0"
  /* 11828 */ "TrackEvent\0"
  /* 11839 */ "TransceiverImpl\0"
  /* 11855 */ "TransitionEvent\0"
  /* 11871 */ "TreeColumn\0"
  /* 11882 */ "TreeColumns\0"
  /* 11894 */ "TreeContentView\0"
  /* 11910 */ "TreeWalker\0"
  /* 11921 */ "U2F\0"
  /* 11925 */ "UDPMessageEvent\0"
  /* 11941 */ "UDPSocket\0"
  /* 11951 */ "UIEvent\0"
  /* 11959 */ "URL\0"
  /* 11963 */ "webkitURL\0"
  /* 11973 */ "URLSearchParams\0"
  /* 11989 */ "UserInteraction\0"
  /* 12005 */ "UserProximityEvent\0"
  /* 12024 */ "VRDisplay\0"
  /* 12034 */ "VRDisplayCapabilities\0"
  /* 12056 */ "VRDisplayEvent\0"
  /* 12071 */ "VREyeParameters\0"
  /* 12087 */ "VRFieldOfView\0"
  /* 12101 */ "VRFrameData\0"
  /* 12113 */ "VRMockController\0"
  /* 12130 */ "VRMockDisplay\0"
  /* 12144 */ "VRPose\0"
  /* 12151 */ "VRServiceTest\0"
  /* 12165 */ "VRStageParameters\0"
  /* 12183 */ "VTTCue\0"
  /* 12190 */ "VTTRegion\0"
  /* 12200 */ "ValidityState\0"
  /* 12214 */ "VideoPlaybackQuality\0"
  /* 12235 */ "VideoTrack\0"
  /* 12246 */ "VideoTrackList\0"
  /* 12261 */ "VisualViewport\0"
  /* 12276 */ "WaveShaperNode\0"
  /* 12291 */ "WebExtensionContentScript\0"
  /* 12317 */ "WebExtensionPolicy\0"
  /* 12336 */ "WebGL2RenderingContext\0"
  /* 12359 */ "WebGLActiveInfo\0"
  /* 12375 */ "WebGLBuffer\0"
  /* 12387 */ "WebGLContextEvent\0"
  /* 12405 */ "WebGLFramebuffer\0"
  /* 12422 */ "WebGLProgram\0"
  /* 12435 */ "WebGLQuery\0"
  /* 12446 */ "WebGLRenderbuffer\0"
  /* 12464 */ "WebGLRenderingContext\0"
  /* 12486 */ "WebGLSampler\0"
  /* 12499 */ "WebGLShader\0"
  /* 12511 */ "WebGLShaderPrecisionFormat\0"
  /* 12538 */ "WebGLSync\0"
  /* 12548 */ "WebGLTexture\0"
  /* 12561 */ "WebGLTransformFeedback\0"
  /* 12584 */ "WebGLUniformLocation\0"
  /* 12605 */ "WebGLVertexArrayObject\0"
  /* 12628 */ "WebSocket\0"
  /* 12638 */ "WebrtcGlobalInformation\0"
  /* 12662 */ "WheelEvent\0"
  /* 12673 */ "Window\0"
  /* 12680 */ "WindowContext\0"
  /* 12694 */ "WindowGlobalChild\0"
  /* 12712 */ "WindowGlobalParent\0"
  /* 12731 */ "WindowRoot\0"
  /* 12742 */ "Worker\0"
  /* 12749 */ "Worklet\0"
  /* 12757 */ "XMLDocument\0"
  /* 12769 */ "XMLHttpRequest\0"
  /* 12784 */ "XMLHttpRequestEventTarget\0"
  /* 12810 */ "XMLHttpRequestUpload\0"
  /* 12831 */ "XMLSerializer\0"
  /* 12845 */ "XPathEvaluator\0"
  /* 12860 */ "XPathExpression\0"
  /* 12876 */ "XPathResult\0"
  /* 12888 */ "XRBoundedReferenceSpace\0"
  /* 12912 */ "XRFrame\0"
  /* 12920 */ "XRInputSource\0"
  /* 12934 */ "XRInputSourceArray\0"
  /* 12953 */ "XRInputSourceEvent\0"
  /* 12972 */ "XRInputSourcesChangeEvent\0"
  /* 12998 */ "XRPose\0"
  /* 13005 */ "XRReferenceSpace\0"
  /* 13022 */ "XRReferenceSpaceEvent\0"
  /* 13044 */ "XRRenderState\0"
  /* 13058 */ "XRRigidTransform\0"
  /* 13075 */ "XRSession\0"
  /* 13085 */ "XRSessionEvent\0"
  /* 13100 */ "XRSpace\0"
  /* 13108 */ "XRSystem\0"
  /* 13117 */ "XRView\0"
  /* 13124 */ "XRViewerPose\0"
  /* 13137 */ "XRViewport\0"
  /* 13148 */ "XRWebGLLayer\0"
  /* 13161 */ "XSLTProcessor\0"
  /* 13175 */ "XULCommandEvent\0"
  /* 13191 */ "XULElement\0"
  /* 13202 */ "XULFrameElement\0"
  /* 13218 */ "XULMenuElement\0"
  /* 13233 */ "XULPopupElement\0"
  /* 13249 */ "XULTextElement\0"
  /* 13264 */ "XULTreeElement\0"
  /* 13279 */ "console\0"
  /* 13287 */ "mozRTCIceCandidate\0"
  /* 13306 */ "mozRTCPeerConnection\0"
  /* 13327 */ "mozRTCSessionDescription\0";

const WebIDLNameTableEntry WebIDLGlobalNameHash::sEntries[] = {
  {
    /* mNameOffset */ 5832, // "MatchGlob"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::MatchGlob,
    /* mCreate */ MatchGlob_Binding::CreateInterfaceObjects,
    /* mEnabled */ MatchGlob_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8070, // "PresentationConnectionAvailableEvent"
    /* mNameLength */ 36,
    /* mConstructorId */ constructors::id::PresentationConnectionAvailableEvent,
    /* mCreate */ PresentationConnectionAvailableEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationConnectionAvailableEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11647, // "TestingDeprecatedInterface"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::TestingDeprecatedInterface,
    /* mCreate */ TestingDeprecatedInterface_Binding::CreateInterfaceObjects,
    /* mEnabled */ TestingDeprecatedInterface_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8220, // "ProcessMessageManager"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::ProcessMessageManager,
    /* mCreate */ ProcessMessageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ ProcessMessageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 181, // "AnimationEvent"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::AnimationEvent,
    /* mCreate */ AnimationEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 589, // "BatteryManager"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::BatteryManager,
    /* mCreate */ BatteryManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ BatteryManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7547, // "PerformanceResourceTiming"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::PerformanceResourceTiming,
    /* mCreate */ PerformanceResourceTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5705, // "Localization"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::Localization,
    /* mCreate */ Localization_Binding::CreateInterfaceObjects,
    /* mEnabled */ Localization_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2025, // "DataTransferItem"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::DataTransferItem,
    /* mCreate */ DataTransferItem_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5351, // "IdleDeadline"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::IdleDeadline,
    /* mCreate */ IdleDeadline_Binding::CreateInterfaceObjects,
    /* mEnabled */ IdleDeadline_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1235, // "CaretStateChangedEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::CaretStateChangedEvent,
    /* mCreate */ CaretStateChangedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ CaretStateChangedEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12446, // "WebGLRenderbuffer"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::WebGLRenderbuffer,
    /* mCreate */ WebGLRenderbuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLRenderbuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1707, // "Credential"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::Credential,
    /* mCreate */ Credential_Binding::CreateInterfaceObjects,
    /* mEnabled */ Credential_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2817, // "GPUBindGroup"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::GPUBindGroup,
    /* mCreate */ GPUBindGroup_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUBindGroup_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1900, // "DOMPointReadOnly"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::DOMPointReadOnly,
    /* mCreate */ DOMPointReadOnly_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMPointReadOnly_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5594, // "JSProcessActorParent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::JSProcessActorParent,
    /* mCreate */ JSProcessActorParent_Binding::CreateInterfaceObjects,
    /* mEnabled */ JSProcessActorParent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4500, // "HTMLOptionElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLOptionElement,
    /* mCreate */ HTMLOptionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11941, // "UDPSocket"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::UDPSocket,
    /* mCreate */ UDPSocket_Binding::CreateInterfaceObjects,
    /* mEnabled */ UDPSocket_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9933, // "SVGGradientElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGGradientElement,
    /* mCreate */ SVGGradientElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3684, // "HTMLAnchorElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLAnchorElement,
    /* mCreate */ HTMLAnchorElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1739, // "Crypto"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Crypto,
    /* mCreate */ Crypto_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1477, // "Clipboard"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Clipboard,
    /* mCreate */ Clipboard_Binding::CreateInterfaceObjects,
    /* mEnabled */ Clipboard_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13161, // "XSLTProcessor"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::XSLTProcessor,
    /* mCreate */ XSLTProcessor_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5903, // "MediaCapabilitiesInfo"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::MediaCapabilitiesInfo,
    /* mCreate */ MediaCapabilitiesInfo_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaCapabilitiesInfo_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 847, // "CSSImportRule"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CSSImportRule,
    /* mCreate */ CSSImportRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1117, // "CanonicalBrowsingContext"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::CanonicalBrowsingContext,
    /* mCreate */ CanonicalBrowsingContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ CanonicalBrowsingContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4136, // "HTMLHeadElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLHeadElement,
    /* mCreate */ HTMLHeadElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3718, // "HTMLAudioElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLAudioElement,
    /* mCreate */ HTMLAudioElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12130, // "VRMockDisplay"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::VRMockDisplay,
    /* mCreate */ VRMockDisplay_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRMockDisplay_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12235, // "VideoTrack"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::VideoTrack,
    /* mCreate */ VideoTrack_Binding::CreateInterfaceObjects,
    /* mEnabled */ VideoTrack_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1545, // "CommandEvent"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CommandEvent,
    /* mCreate */ CommandEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ CommandEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12810, // "XMLHttpRequestUpload"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::XMLHttpRequestUpload,
    /* mCreate */ XMLHttpRequestUpload_Binding::CreateInterfaceObjects,
    /* mEnabled */ XMLHttpRequestUpload_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7729, // "PlacesFavicon"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::PlacesFavicon,
    /* mCreate */ PlacesFavicon_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesFavicon_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6942, // "Notification"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::Notification,
    /* mCreate */ Notification_Binding::CreateInterfaceObjects,
    /* mEnabled */ Notification_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9456, // "SVGFEDisplacementMapElement"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::SVGFEDisplacementMapElement,
    /* mCreate */ SVGFEDisplacementMapElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 831, // "CSSGroupingRule"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::CSSGroupingRule,
    /* mCreate */ CSSGroupingRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12276, // "WaveShaperNode"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::WaveShaperNode,
    /* mCreate */ WaveShaperNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ WaveShaperNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7040, // "OffscreenCanvas"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::OffscreenCanvas,
    /* mCreate */ OffscreenCanvas_Binding::CreateInterfaceObjects,
    /* mEnabled */ OffscreenCanvas_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7160, // "Path2D"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Path2D,
    /* mCreate */ Path2D_Binding::CreateInterfaceObjects,
    /* mEnabled */ Path2D_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9997, // "SVGLengthList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGLengthList,
    /* mCreate */ SVGLengthList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10727, // "ScrollViewChangeEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::ScrollViewChangeEvent,
    /* mCreate */ ScrollViewChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ ScrollViewChangeEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1881, // "DOMParser"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::DOMParser,
    /* mCreate */ DOMParser_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11593, // "TCPSocketErrorEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::TCPSocketErrorEvent,
    /* mCreate */ TCPSocketErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ TCPSocketErrorEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6816, // "MutationEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MutationEvent,
    /* mCreate */ MutationEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3115, // "GPURenderPassEncoder"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::GPURenderPassEncoder,
    /* mCreate */ GPURenderPassEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPURenderPassEncoder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6501, // "MessageListenerManager"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::MessageListenerManager,
    /* mCreate */ MessageListenerManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ MessageListenerManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11396, // "StructuredCloneHolder"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::StructuredCloneHolder,
    /* mCreate */ StructuredCloneHolder_Binding::CreateInterfaceObjects,
    /* mEnabled */ StructuredCloneHolder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9248, // "SVGDefsElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGDefsElement,
    /* mCreate */ SVGDefsElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11451, // "StyleSheetApplicableStateChangeEvent"
    /* mNameLength */ 36,
    /* mConstructorId */ constructors::id::StyleSheetApplicableStateChangeEvent,
    /* mCreate */ StyleSheetApplicableStateChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ StyleSheetApplicableStateChangeEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9278, // "SVGElement"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::SVGElement,
    /* mCreate */ SVGElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6188, // "MediaQueryListEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MediaQueryListEvent,
    /* mCreate */ MediaQueryListEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10454, // "SVGTSpanElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGTSpanElement,
    /* mCreate */ SVGTSpanElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 790, // "CSSFontFaceRule"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::CSSFontFaceRule,
    /* mCreate */ CSSFontFaceRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7951, // "PopupPositionedEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::PopupPositionedEvent,
    /* mCreate */ PopupPositionedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PopupPositionedEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2437, // "EventTarget"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::EventTarget,
    /* mCreate */ EventTarget_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10893, // "SimpleGestureEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SimpleGestureEvent,
    /* mCreate */ SimpleGestureEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ SimpleGestureEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10552, // "SVGTitleElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGTitleElement,
    /* mCreate */ SVGTitleElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 907, // "CSSMozDocumentRule"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::CSSMozDocumentRule,
    /* mCreate */ CSSMozDocumentRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8857, // "SVGAnimateMotionElement"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SVGAnimateMotionElement,
    /* mCreate */ SVGAnimateMotionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3755, // "HTMLBaseElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLBaseElement,
    /* mCreate */ HTMLBaseElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1821, // "DOMLocalization"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::DOMLocalization,
    /* mCreate */ DOMLocalization_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMLocalization_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6038, // "MediaError"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::MediaError,
    /* mCreate */ MediaError_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3434, // "GamepadServiceTest"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::GamepadServiceTest,
    /* mCreate */ GamepadServiceTest_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadServiceTest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8188, // "PresentationRequest"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::PresentationRequest,
    /* mCreate */ PresentationRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationRequest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6660, // "MozQueryInterface"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::MozQueryInterface,
    /* mCreate */ MozQueryInterface_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozQueryInterface_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5101, // "History"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::History,
    /* mCreate */ History_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5718, // "Location"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::Location,
    /* mCreate */ Location_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12317, // "WebExtensionPolicy"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::WebExtensionPolicy,
    /* mCreate */ WebExtensionPolicy_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebExtensionPolicy_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3251, // "GPUUncapturedErrorEvent"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::GPUUncapturedErrorEvent,
    /* mCreate */ GPUUncapturedErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUUncapturedErrorEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 639, // "Blob"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Blob,
    /* mCreate */ Blob_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8770, // "ResizeObserverEntry"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::ResizeObserverEntry,
    /* mCreate */ ResizeObserverEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ ResizeObserverEntry_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6904, // "Node"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Node,
    /* mCreate */ Node_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11839, // "TransceiverImpl"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::TransceiverImpl,
    /* mCreate */ TransceiverImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ TransceiverImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11628, // "TelemetryStopwatch"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::TelemetryStopwatch,
    /* mCreate */ TelemetryStopwatch_Binding::CreateInterfaceObjects,
    /* mEnabled */ TelemetryStopwatch_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4777, // "HTMLTableCaptionElement"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::HTMLTableCaptionElement,
    /* mCreate */ HTMLTableCaptionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5440, // "ImageData"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::ImageData,
    /* mCreate */ ImageData_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3989, // "HTMLEmbedElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLEmbedElement,
    /* mCreate */ HTMLEmbedElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6955, // "NotifyPaintEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::NotifyPaintEvent,
    /* mCreate */ NotifyPaintEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ NotifyPaintEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5083, // "HiddenPluginEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HiddenPluginEvent,
    /* mCreate */ HiddenPluginEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ HiddenPluginEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10470, // "SVGTextContentElement"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGTextContentElement,
    /* mCreate */ SVGTextContentElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12336, // "WebGL2RenderingContext"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::WebGL2RenderingContext,
    /* mCreate */ WebGL2RenderingContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGL2RenderingContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13005, // "XRReferenceSpace"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::XRReferenceSpace,
    /* mCreate */ XRReferenceSpace_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRReferenceSpace_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2123, // "DeprecationReportBody"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::DeprecationReportBody,
    /* mCreate */ DeprecationReportBody_Binding::CreateInterfaceObjects,
    /* mEnabled */ DeprecationReportBody_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1566, // "CompositionEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::CompositionEvent,
    /* mCreate */ CompositionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9705, // "SVGFEMorphologyElement"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGFEMorphologyElement,
    /* mCreate */ SVGFEMorphologyElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8925, // "SVGAnimatedBoolean"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGAnimatedBoolean,
    /* mCreate */ SVGAnimatedBoolean_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12422, // "WebGLProgram"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::WebGLProgram,
    /* mCreate */ WebGLProgram_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLProgram_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8022, // "PresentationAvailability"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::PresentationAvailability,
    /* mCreate */ PresentationAvailability_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationAvailability_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7475, // "PerformanceObserver"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::PerformanceObserver,
    /* mCreate */ PerformanceObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13264, // "XULTreeElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XULTreeElement,
    /* mCreate */ XULTreeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULTreeElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1452, // "ChromeUtils"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::ChromeUtils,
    /* mCreate */ ChromeUtils_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChromeUtils_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12144, // "VRPose"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::VRPose,
    /* mCreate */ VRPose_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRPose_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8140, // "PresentationConnectionList"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::PresentationConnectionList,
    /* mCreate */ PresentationConnectionList_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationConnectionList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2042, // "DataTransferItemList"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::DataTransferItemList,
    /* mCreate */ DataTransferItemList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2681, // "FluentResource"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::FluentResource,
    /* mCreate */ FluentResource_Binding::CreateInterfaceObjects,
    /* mEnabled */ FluentResource_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1778, // "CustomEvent"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::CustomEvent,
    /* mCreate */ CustomEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1746, // "CryptoKey"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::CryptoKey,
    /* mCreate */ CryptoKey_Binding::CreateInterfaceObjects,
    /* mEnabled */ CryptoKey_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11488, // "StyleSheetList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::StyleSheetList,
    /* mCreate */ StyleSheetList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10880, // "SharedWorker"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SharedWorker,
    /* mCreate */ SharedWorker_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2145, // "DeviceLightEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::DeviceLightEvent,
    /* mCreate */ DeviceLightEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ DeviceLightEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9770, // "SVGFESpecularLightingElement"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::SVGFESpecularLightingElement,
    /* mCreate */ SVGFESpecularLightingElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8662, // "RTCStatsReport"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::RTCStatsReport,
    /* mCreate */ RTCStatsReport_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCStatsReport_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4445, // "HTMLOListElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLOListElement,
    /* mCreate */ HTMLOListElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13022, // "XRReferenceSpaceEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::XRReferenceSpaceEvent,
    /* mCreate */ XRReferenceSpaceEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRReferenceSpaceEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4565, // "HTMLParagraphElement"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::HTMLParagraphElement,
    /* mCreate */ HTMLParagraphElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2482, // "FetchObserver"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::FetchObserver,
    /* mCreate */ FetchObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ FetchObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11018, // "SpeechRecognition"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SpeechRecognition,
    /* mCreate */ SpeechRecognition_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognition_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4657, // "HTMLQuoteElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLQuoteElement,
    /* mCreate */ HTMLQuoteElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3626, // "GridLine"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::GridLine,
    /* mCreate */ GridLine_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridLine_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10011, // "SVGLineElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGLineElement,
    /* mCreate */ SVGLineElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11818, // "TouchList"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::TouchList,
    /* mCreate */ TouchList_Binding::CreateInterfaceObjects,
    /* mEnabled */ TouchList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9044, // "SVGAnimatedNumberList"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGAnimatedNumberList,
    /* mCreate */ SVGAnimatedNumberList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5493, // "InspectorUtils"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::InspectorUtils,
    /* mCreate */ InspectorUtils_Binding::CreateInterfaceObjects,
    /* mEnabled */ InspectorUtils_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 118, // "AddonManagerPermissions"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::AddonManagerPermissions,
    /* mCreate */ AddonManagerPermissions_Binding::CreateInterfaceObjects,
    /* mEnabled */ AddonManagerPermissions_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5234, // "IDBMutableFile"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::IDBMutableFile,
    /* mCreate */ IDBMutableFile_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13279, // "console"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::console,
    /* mCreate */ console_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12757, // "XMLDocument"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::XMLDocument,
    /* mCreate */ XMLDocument_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2806, // "GPUAdapter"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::GPUAdapter,
    /* mCreate */ GPUAdapter_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUAdapter_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13148, // "XRWebGLLayer"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::XRWebGLLayer,
    /* mCreate */ XRWebGLLayer_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRWebGLLayer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4710, // "HTMLSlotElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLSlotElement,
    /* mCreate */ HTMLSlotElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8691, // "RadioNodeList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::RadioNodeList,
    /* mCreate */ RadioNodeList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3154, // "GPUSampler"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::GPUSampler,
    /* mCreate */ GPUSampler_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUSampler_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3546, // "GleanCategory"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::GleanCategory,
    /* mCreate */ GleanCategory_Binding::CreateInterfaceObjects,
    /* mEnabled */ GleanCategory_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4822, // "HTMLTableColElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLTableColElement,
    /* mCreate */ HTMLTableColElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10375, // "SVGStopElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGStopElement,
    /* mCreate */ SVGStopElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6430, // "MerchantValidationEvent"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::MerchantValidationEvent,
    /* mCreate */ MerchantValidationEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ MerchantValidationEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11562, // "TCPServerSocketEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::TCPServerSocketEvent,
    /* mCreate */ TCPServerSocketEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ TCPServerSocketEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2234, // "Document"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::Document,
    /* mCreate */ Document_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10084, // "SVGMaskElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGMaskElement,
    /* mCreate */ SVGMaskElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4903, // "HTMLTemplateElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLTemplateElement,
    /* mCreate */ HTMLTemplateElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12920, // "XRInputSource"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::XRInputSource,
    /* mCreate */ XRInputSource_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRInputSource_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10581, // "SVGTransformList"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGTransformList,
    /* mCreate */ SVGTransformList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6118, // "MediaKeySystemAccess"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::MediaKeySystemAccess,
    /* mCreate */ MediaKeySystemAccess_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2337, // "Element"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Element,
    /* mCreate */ Element_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 992, // "CSSStyleDeclaration"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::CSSStyleDeclaration,
    /* mCreate */ CSSStyleDeclaration_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7316, // "Performance"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::Performance,
    /* mCreate */ Performance_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1296, // "ChannelWrapper"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::ChannelWrapper,
    /* mCreate */ ChannelWrapper_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChannelWrapper_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2962, // "GPUComputePipeline"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::GPUComputePipeline,
    /* mCreate */ GPUComputePipeline_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUComputePipeline_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9861, // "SVGFilterElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGFilterElement,
    /* mCreate */ SVGFilterElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5768, // "MIDIInputMap"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MIDIInputMap,
    /* mCreate */ MIDIInputMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIInputMap_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3655, // "GridTracks"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::GridTracks,
    /* mCreate */ GridTracks_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridTracks_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8295, // "PromiseRejectionEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::PromiseRejectionEvent,
    /* mCreate */ PromiseRejectionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6933, // "NodeList"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::NodeList,
    /* mCreate */ NodeList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5508, // "InstallTriggerImpl"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::InstallTriggerImpl,
    /* mCreate */ InstallTriggerImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ InstallTriggerImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12538, // "WebGLSync"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::WebGLSync,
    /* mCreate */ WebGLSync_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLSync_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 165, // "AnimationEffect"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::AnimationEffect,
    /* mCreate */ AnimationEffect_Binding::CreateInterfaceObjects,
    /* mEnabled */ AnimationEffect_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1847, // "WebKitCSSMatrix"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::DOMMatrix,
    /* mCreate */ DOMMatrix_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMMatrix_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2922, // "GPUCommandEncoder"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::GPUCommandEncoder,
    /* mCreate */ GPUCommandEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUCommandEncoder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13085, // "XRSessionEvent"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XRSessionEvent,
    /* mCreate */ XRSessionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRSessionEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5376, // "ImageBitmapRenderingContext"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::ImageBitmapRenderingContext,
    /* mCreate */ ImageBitmapRenderingContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11036, // "webkitSpeechRecognition"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SpeechRecognition,
    /* mCreate */ SpeechRecognition_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognition_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3311, // "GamepadAxisMoveEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::GamepadAxisMoveEvent,
    /* mCreate */ GamepadAxisMoveEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadAxisMoveEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3570, // "GleanLabeled"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::GleanLabeled,
    /* mCreate */ GleanLabeled_Binding::CreateInterfaceObjects,
    /* mEnabled */ GleanLabeled_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9878, // "SVGForeignObjectElement"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SVGForeignObjectElement,
    /* mCreate */ SVGForeignObjectElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12190, // "VTTRegion"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::VTTRegion,
    /* mCreate */ VTTRegion_Binding::CreateInterfaceObjects,
    /* mEnabled */ VTTRegion_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7121, // "PannerNode"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::PannerNode,
    /* mCreate */ PannerNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ PannerNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 806, // "CSSFontFeatureValuesRule"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::CSSFontFeatureValuesRule,
    /* mCreate */ CSSFontFeatureValuesRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7495, // "PerformanceObserverEntryList"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::PerformanceObserverEntryList,
    /* mCreate */ PerformanceObserverEntryList_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceObserverEntryList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13249, // "XULTextElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XULTextElement,
    /* mCreate */ XULTextElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULTextElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2667, // "FluentPattern"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::FluentPattern,
    /* mCreate */ FluentPattern_Binding::CreateInterfaceObjects,
    /* mEnabled */ FluentPattern_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1999, // "DOMTokenList"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::DOMTokenList,
    /* mCreate */ DOMTokenList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6159, // "MediaMetadata"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MediaMetadata,
    /* mCreate */ MediaMetadata_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaMetadata_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2379, // "EventCallbackDebuggerNotification"
    /* mNameLength */ 33,
    /* mConstructorId */ constructors::id::EventCallbackDebuggerNotification,
    /* mCreate */ EventCallbackDebuggerNotification_Binding::CreateInterfaceObjects,
    /* mEnabled */ EventCallbackDebuggerNotification_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5615, // "JSWindowActorChild"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::JSWindowActorChild,
    /* mCreate */ JSWindowActorChild_Binding::CreateInterfaceObjects,
    /* mEnabled */ JSWindowActorChild_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12499, // "WebGLShader"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::WebGLShader,
    /* mCreate */ WebGLShader_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLShader_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1558, // "Comment"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Comment,
    /* mCreate */ Comment_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3964, // "HTMLDocument"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::HTMLDocument,
    /* mCreate */ HTMLDocument_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4245, // "HTMLLIElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::HTMLLIElement,
    /* mCreate */ HTMLLIElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12638, // "WebrtcGlobalInformation"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::WebrtcGlobalInformation,
    /* mCreate */ WebrtcGlobalInformation_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebrtcGlobalInformation_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2277, // "DocumentType"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::DocumentType,
    /* mCreate */ DocumentType_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4026, // "HTMLFontElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLFontElement,
    /* mCreate */ HTMLFontElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10650, // "Screen"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Screen,
    /* mCreate */ Screen_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3029, // "GPUOutOfMemoryError"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::GPUOutOfMemoryError,
    /* mCreate */ GPUOutOfMemoryError_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUOutOfMemoryError_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6063, // "MediaKeyMessageEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::MediaKeyMessageEvent,
    /* mCreate */ MediaKeyMessageEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13287, // "mozRTCIceCandidate"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::mozRTCIceCandidate,
    /* mCreate */ mozRTCIceCandidate_Binding::CreateInterfaceObjects,
    /* mEnabled */ mozRTCIceCandidate_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8809, // "Response"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::Response,
    /* mCreate */ Response_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11766, // "TextTrackList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::TextTrackList,
    /* mCreate */ TextTrackList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5925, // "MediaControlService"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MediaControlService,
    /* mCreate */ MediaControlService_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaControlService_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8047, // "PresentationConnection"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::PresentationConnection,
    /* mCreate */ PresentationConnection_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationConnection_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5634, // "JSWindowActorParent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::JSWindowActorParent,
    /* mCreate */ JSWindowActorParent_Binding::CreateInterfaceObjects,
    /* mEnabled */ JSWindowActorParent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1583, // "ConsoleInstance"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::ConsoleInstance,
    /* mCreate */ ConsoleInstance_Binding::CreateInterfaceObjects,
    /* mEnabled */ ConsoleInstance_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8421, // "RTCDTMFSender"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::RTCDTMFSender,
    /* mCreate */ RTCDTMFSender_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11855, // "TransitionEvent"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::TransitionEvent,
    /* mCreate */ TransitionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2981, // "GPUDevice"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::GPUDevice,
    /* mCreate */ GPUDevice_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUDevice_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8594, // "RTCRtpReceiver"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::RTCRtpReceiver,
    /* mCreate */ RTCRtpReceiver_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCRtpReceiver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3560, // "GleanImpl"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::GleanImpl,
    /* mCreate */ GleanImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ GleanImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9379, // "SVGFECompositeElement"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGFECompositeElement,
    /* mCreate */ SVGFECompositeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7919, // "PopStateEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::PopStateEvent,
    /* mCreate */ PopStateEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2425, // "EventSource"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::EventSource,
    /* mCreate */ EventSource_Binding::CreateInterfaceObjects,
    /* mEnabled */ EventSource_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8640, // "RTCSessionDescription"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::RTCSessionDescription,
    /* mCreate */ RTCSessionDescription_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCSessionDescription_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10640, // "Sanitizer"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Sanitizer,
    /* mCreate */ Sanitizer_Binding::CreateInterfaceObjects,
    /* mEnabled */ Sanitizer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4525, // "HTMLOptionsCollection"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::HTMLOptionsCollection,
    /* mCreate */ HTMLOptionsCollection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7743, // "PlacesHistoryCleared"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::PlacesHistoryCleared,
    /* mCreate */ PlacesHistoryCleared_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesHistoryCleared_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12214, // "VideoPlaybackQuality"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::VideoPlaybackQuality,
    /* mCreate */ VideoPlaybackQuality_Binding::CreateInterfaceObjects,
    /* mEnabled */ VideoPlaybackQuality_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 708, // "CDATASection"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CDATASection,
    /* mCreate */ CDATASection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 644, // "BlobEvent"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::BlobEvent,
    /* mCreate */ BlobEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1417, // "ChromeMessageSender"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::ChromeMessageSender,
    /* mCreate */ ChromeMessageSender_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChromeMessageSender_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12024, // "VRDisplay"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::VRDisplay,
    /* mCreate */ VRDisplay_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRDisplay_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9066, // "SVGAnimatedPreserveAspectRatio"
    /* mNameLength */ 30,
    /* mConstructorId */ constructors::id::SVGAnimatedPreserveAspectRatio,
    /* mCreate */ SVGAnimatedPreserveAspectRatio_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5138, // "IDBDatabase"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::IDBDatabase,
    /* mCreate */ IDBDatabase_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6049, // "MediaKeyError"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MediaKeyError,
    /* mCreate */ MediaKeyError_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2619, // "Flex"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Flex,
    /* mCreate */ Flex_Binding::CreateInterfaceObjects,
    /* mEnabled */ Flex_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11925, // "UDPMessageEvent"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::UDPMessageEvent,
    /* mCreate */ UDPMessageEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ UDPMessageEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8337, // "PushManager"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::PushManager,
    /* mCreate */ PushManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ PushManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4102, // "HTMLFrameSetElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLFrameSetElement,
    /* mCreate */ HTMLFrameSetElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11726, // "TextTrack"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::TextTrack,
    /* mCreate */ TextTrack_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8406, // "RTCCertificate"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::RTCCertificate,
    /* mCreate */ RTCCertificate_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCCertificate_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11749, // "TextTrackCueList"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::TextTrackCueList,
    /* mCreate */ TextTrackCueList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1803, // "DOMImplementation"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::DOMImplementation,
    /* mCreate */ DOMImplementation_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3871, // "HTMLDataListElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLDataListElement,
    /* mCreate */ HTMLDataListElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8677, // "RTCTrackEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::RTCTrackEvent,
    /* mCreate */ RTCTrackEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCTrackEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3666, // "HTMLAllCollection"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLAllCollection,
    /* mCreate */ HTMLAllCollection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4801, // "HTMLTableCellElement"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::HTMLTableCellElement,
    /* mCreate */ HTMLTableCellElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2758, // "FormDataEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::FormDataEvent,
    /* mCreate */ FormDataEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4480, // "HTMLOptGroupElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLOptGroupElement,
    /* mCreate */ HTMLOptGroupElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10711, // "ScrollAreaEvent"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::ScrollAreaEvent,
    /* mCreate */ ScrollAreaEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1196, // "CanvasRenderingContext2D"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::CanvasRenderingContext2D,
    /* mCreate */ CanvasRenderingContext2D_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8622, // "RTCRtpTransceiver"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::RTCRtpTransceiver,
    /* mCreate */ RTCRtpTransceiver_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCRtpTransceiver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2203, // "DeviceProximityEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::DeviceProximityEvent,
    /* mCreate */ DeviceProximityEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ DeviceProximityEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 259, // "AudioBuffer"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::AudioBuffer,
    /* mCreate */ AudioBuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioBuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6972, // "OfflineAudioCompletionEvent"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::OfflineAudioCompletionEvent,
    /* mCreate */ OfflineAudioCompletionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ OfflineAudioCompletionEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8609, // "RTCRtpSender"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::RTCRtpSender,
    /* mCreate */ RTCRtpSender_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCRtpSender_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4637, // "HTMLProgressElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLProgressElement,
    /* mCreate */ HTMLProgressElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1039, // "CSSSupportsRule"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::CSSSupportsRule,
    /* mCreate */ CSSSupportsRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5727, // "MIDIAccess"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::MIDIAccess,
    /* mCreate */ MIDIAccess_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIAccess_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7425, // "PerformanceNavigation"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::PerformanceNavigation,
    /* mCreate */ PerformanceNavigation_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3635, // "GridLines"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::GridLines,
    /* mCreate */ GridLines_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridLines_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 351, // "AudioParam"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::AudioParam,
    /* mCreate */ AudioParam_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioParam_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5199, // "IDBKeyRange"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::IDBKeyRange,
    /* mCreate */ IDBKeyRange_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 0, // "APZHitResultFlags"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::APZHitResultFlags,
    /* mCreate */ APZHitResultFlags_Binding::CreateInterfaceObjects,
    /* mEnabled */ APZHitResultFlags_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11674, // "Text"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Text,
    /* mCreate */ Text_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11963, // "webkitURL"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::URL,
    /* mCreate */ URL_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11807, // "TouchEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TouchEvent,
    /* mCreate */ TouchEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ TouchEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8009, // "Presentation"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::Presentation,
    /* mCreate */ Presentation_Binding::CreateInterfaceObjects,
    /* mEnabled */ Presentation_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6920, // "NodeIterator"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::NodeIterator,
    /* mCreate */ NodeIterator_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11528, // "SyncMessageSender"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SyncMessageSender,
    /* mCreate */ SyncMessageSender_Binding::CreateInterfaceObjects,
    /* mEnabled */ SyncMessageSender_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5175, // "IDBFileRequest"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::IDBFileRequest,
    /* mCreate */ IDBFileRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10526, // "SVGTextPositioningElement"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::SVGTextPositioningElement,
    /* mCreate */ SVGTextPositioningElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1516, // "ClonedErrorHolder"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::ClonedErrorHolder,
    /* mCreate */ ClonedErrorHolder_Binding::CreateInterfaceObjects,
    /* mEnabled */ ClonedErrorHolder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2728, // "FontFaceSetLoadEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::FontFaceSetLoadEvent,
    /* mCreate */ FontFaceSetLoadEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ FontFaceSetLoadEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4069, // "HTMLFormElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLFormElement,
    /* mCreate */ HTMLFormElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12071, // "VREyeParameters"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::VREyeParameters,
    /* mCreate */ VREyeParameters_Binding::CreateInterfaceObjects,
    /* mEnabled */ VREyeParameters_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1378, // "ChildSHistory"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ChildSHistory,
    /* mCreate */ ChildSHistory_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChildSHistory_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1012, // "CSSStyleRule"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CSSStyleRule,
    /* mCreate */ CSSStyleRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 972, // "CSSRule"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::CSSRule,
    /* mCreate */ CSSRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2532, // "FileSystemDirectoryEntry"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::FileSystemDirectoryEntry,
    /* mCreate */ FileSystemDirectoryEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12200, // "ValidityState"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ValidityState,
    /* mCreate */ ValidityState_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10099, // "SVGMatrix"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::SVGMatrix,
    /* mCreate */ SVGMatrix_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2891, // "GPUColorWrite"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::GPUColorWrite,
    /* mCreate */ GPUColorWrite_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUColorWrite_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5150, // "IDBFactory"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::IDBFactory,
    /* mCreate */ IDBFactory_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1311, // "CharacterData"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CharacterData,
    /* mCreate */ CharacterData_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3209, // "GPUTexture"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::GPUTexture,
    /* mCreate */ GPUTexture_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUTexture_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5871, // "MathMLElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MathMLElement,
    /* mCreate */ MathMLElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11112, // "SpeechRecognitionEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SpeechRecognitionEvent,
    /* mCreate */ SpeechRecognitionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognitionEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 877, // "CSSKeyframesRule"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::CSSKeyframesRule,
    /* mCreate */ CSSKeyframesRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7000, // "OfflineAudioContext"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::OfflineAudioContext,
    /* mCreate */ OfflineAudioContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ OfflineAudioContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5249, // "IDBObjectStore"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::IDBObjectStore,
    /* mCreate */ IDBObjectStore_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 92, // "AddonInstall"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::AddonInstall,
    /* mCreate */ AddonInstall_Binding::CreateInterfaceObjects,
    /* mEnabled */ AddonInstall_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9156, // "SVGAnimationElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::SVGAnimationElement,
    /* mCreate */ SVGAnimationElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11871, // "TreeColumn"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TreeColumn,
    /* mCreate */ TreeColumn_Binding::CreateInterfaceObjects,
    /* mEnabled */ TreeColumn_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6524, // "MessagePort"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::MessagePort,
    /* mCreate */ MessagePort_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2639, // "FlexLineValues"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::FlexLineValues,
    /* mCreate */ FlexLineValues_Binding::CreateInterfaceObjects,
    /* mEnabled */ FlexLineValues_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 478, // "AuthenticatorAssertionResponse"
    /* mNameLength */ 30,
    /* mConstructorId */ constructors::id::AuthenticatorAssertionResponse,
    /* mCreate */ AuthenticatorAssertionResponse_Binding::CreateInterfaceObjects,
    /* mEnabled */ AuthenticatorAssertionResponse_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11613, // "TCPSocketEvent"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::TCPSocketEvent,
    /* mCreate */ TCPSocketEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ TCPSocketEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7328, // "PerformanceEntry"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PerformanceEntry,
    /* mCreate */ PerformanceEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8830, // "SVGAngle"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::SVGAngle,
    /* mCreate */ SVGAngle_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1986, // "DOMStringMap"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::DOMStringMap,
    /* mCreate */ DOMStringMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 422, // "AudioTrack"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::AudioTrack,
    /* mCreate */ AudioTrack_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioTrack_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13058, // "XRRigidTransform"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::XRRigidTransform,
    /* mCreate */ XRRigidTransform_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRRigidTransform_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3702, // "HTMLAreaElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLAreaElement,
    /* mCreate */ HTMLAreaElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12486, // "WebGLSampler"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::WebGLSampler,
    /* mCreate */ WebGLSampler_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLSampler_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12405, // "WebGLFramebuffer"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::WebGLFramebuffer,
    /* mCreate */ WebGLFramebuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLFramebuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1221, // "CaretPosition"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CaretPosition,
    /* mCreate */ CaretPosition_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5364, // "ImageBitmap"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::ImageBitmap,
    /* mCreate */ ImageBitmap_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7192, // "PaymentMethodChangeEvent"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::PaymentMethodChangeEvent,
    /* mCreate */ PaymentMethodChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PaymentMethodChangeEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7780, // "PlacesRanking"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::PlacesRanking,
    /* mCreate */ PlacesRanking_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesRanking_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6909, // "NodeFilter"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::NodeFilter,
    /* mCreate */ NodeFilter_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 753, // "CSSConditionRule"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::CSSConditionRule,
    /* mCreate */ CSSConditionRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10026, // "SVGLinearGradientElement"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SVGLinearGradientElement,
    /* mCreate */ SVGLinearGradientElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4042, // "HTMLFormControlsCollection"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::HTMLFormControlsCollection,
    /* mCreate */ HTMLFormControlsCollection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 770, // "CSSCounterStyleRule"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::CSSCounterStyleRule,
    /* mCreate */ CSSCounterStyleRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4205, // "HTMLImageElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLImageElement,
    /* mCreate */ HTMLImageElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2413, // "EventCounts"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::EventCounts,
    /* mCreate */ EventCounts_Binding::CreateInterfaceObjects,
    /* mEnabled */ EventCounts_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7447, // "PerformanceNavigationTiming"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::PerformanceNavigationTiming,
    /* mCreate */ PerformanceNavigationTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceNavigationTiming_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7717, // "PlacesEvent"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::PlacesEvent,
    /* mCreate */ PlacesEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4344, // "HTMLMediaElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLMediaElement,
    /* mCreate */ HTMLMediaElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12876, // "XPathResult"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::XPathResult,
    /* mCreate */ XPathResult_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10315, // "SVGRectElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGRectElement,
    /* mCreate */ SVGRectElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2557, // "FileSystemDirectoryReader"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::FileSystemDirectoryReader,
    /* mCreate */ FileSystemDirectoryReader_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7020, // "OfflineResourceList"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::OfflineResourceList,
    /* mCreate */ OfflineResourceList_Binding::CreateInterfaceObjects,
    /* mEnabled */ OfflineResourceList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11583, // "TCPSocket"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::TCPSocket,
    /* mCreate */ TCPSocket_Binding::CreateInterfaceObjects,
    /* mEnabled */ TCPSocket_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3735, // "Audio"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::HTMLAudioElement,
    /* mCreate */ HTMLAudioElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1258, // "ChannelMergerNode"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::ChannelMergerNode,
    /* mCreate */ ChannelMergerNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChannelMergerNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3583, // "GleanPingsImpl"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::GleanPingsImpl,
    /* mCreate */ GleanPingsImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ GleanPingsImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9550, // "SVGFEFuncAElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEFuncAElement,
    /* mCreate */ SVGFEFuncAElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9902, // "SVGGElement"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::SVGGElement,
    /* mCreate */ SVGGElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1075, // "CacheStorage"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CacheStorage,
    /* mCreate */ CacheStorage_Binding::CreateInterfaceObjects,
    /* mEnabled */ CacheStorage_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3422, // "GamepadPose"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::GamepadPose,
    /* mCreate */ GamepadPose_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadPose_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4187, // "HTMLIFrameElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLIFrameElement,
    /* mCreate */ HTMLIFrameElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5029, // "HTMLVideoElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLVideoElement,
    /* mCreate */ HTMLVideoElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2583, // "FileSystemEntry"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::FileSystemEntry,
    /* mCreate */ FileSystemEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4006, // "HTMLFieldSetElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLFieldSetElement,
    /* mCreate */ HTMLFieldSetElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10657, // "ScreenLuminance"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::ScreenLuminance,
    /* mCreate */ ScreenLuminance_Binding::CreateInterfaceObjects,
    /* mEnabled */ ScreenLuminance_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2501, // "FileList"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::FileList,
    /* mCreate */ FileList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4228, // "HTMLInputElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLInputElement,
    /* mCreate */ HTMLInputElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7887, // "PluginCrashedEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::PluginCrashedEvent,
    /* mCreate */ PluginCrashedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PluginCrashedEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12673, // "Window"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Window,
    /* mCreate */ Window_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2260, // "DocumentTimeline"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::DocumentTimeline,
    /* mCreate */ DocumentTimeline_Binding::CreateInterfaceObjects,
    /* mEnabled */ DocumentTimeline_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5070, // "HeapSnapshot"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::HeapSnapshot,
    /* mCreate */ HeapSnapshot_Binding::CreateInterfaceObjects,
    /* mEnabled */ HeapSnapshot_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 721, // "CSS"
    /* mNameLength */ 3,
    /* mConstructorId */ constructors::id::CSS,
    /* mCreate */ CSS_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7933, // "PopupBlockedEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::PopupBlockedEvent,
    /* mCreate */ PopupBlockedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12165, // "VRStageParameters"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::VRStageParameters,
    /* mCreate */ VRStageParameters_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRStageParameters_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10825, // "ServiceWorkerRegistration"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::ServiceWorkerRegistration,
    /* mCreate */ ServiceWorkerRegistration_Binding::CreateInterfaceObjects,
    /* mEnabled */ ServiceWorkerRegistration_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10802, // "ServiceWorkerContainer"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::ServiceWorkerContainer,
    /* mCreate */ ServiceWorkerContainer_Binding::CreateInterfaceObjects,
    /* mEnabled */ ServiceWorkerContainer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3400, // "GamepadLightIndicator"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::GamepadLightIndicator,
    /* mCreate */ GamepadLightIndicator_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadLightIndicator_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4586, // "HTMLParamElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLParamElement,
    /* mCreate */ HTMLParamElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2696, // "FocusEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::FocusEvent,
    /* mCreate */ FocusEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7645, // "Permissions"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::Permissions,
    /* mCreate */ Permissions_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12359, // "WebGLActiveInfo"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::WebGLActiveInfo,
    /* mCreate */ WebGLActiveInfo_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLActiveInfo_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8510, // "RTCIceCandidate"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::RTCIceCandidate,
    /* mCreate */ RTCIceCandidate_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCIceCandidate_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2243, // "DocumentFragment"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::DocumentFragment,
    /* mCreate */ DocumentFragment_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5654, // "KeyEvent"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::KeyEvent,
    /* mCreate */ KeyEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4622, // "HTMLPreElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::HTMLPreElement,
    /* mCreate */ HTMLPreElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8107, // "PresentationConnectionCloseEvent"
    /* mNameLength */ 32,
    /* mConstructorId */ constructors::id::PresentationConnectionCloseEvent,
    /* mCreate */ PresentationConnectionCloseEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationConnectionCloseEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13175, // "XULCommandEvent"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::XULCommandEvent,
    /* mCreate */ XULCommandEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULCommandEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4603, // "HTMLPictureElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::HTMLPictureElement,
    /* mCreate */ HTMLPictureElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7868, // "Plugin"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Plugin,
    /* mCreate */ Plugin_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4085, // "HTMLFrameElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLFrameElement,
    /* mCreate */ HTMLFrameElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10507, // "SVGTextPathElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGTextPathElement,
    /* mCreate */ SVGTextPathElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6100, // "MediaKeyStatusMap"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::MediaKeyStatusMap,
    /* mCreate */ MediaKeyStatusMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7345, // "PerformanceEntryEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::PerformanceEntryEvent,
    /* mCreate */ PerformanceEntryEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceEntryEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3049, // "GPUPipelineLayout"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::GPUPipelineLayout,
    /* mCreate */ GPUPipelineLayout_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUPipelineLayout_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10942, // "SpeechGrammar"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SpeechGrammar,
    /* mCreate */ SpeechGrammar_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechGrammar_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 196, // "AnimationPlaybackEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::AnimationPlaybackEvent,
    /* mCreate */ AnimationPlaybackEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ AnimationPlaybackEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 155, // "Animation"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Animation,
    /* mCreate */ Animation_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1674, // "ConvolverNode"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ConvolverNode,
    /* mCreate */ ConvolverNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ ConvolverNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11229, // "SpeechSynthesisEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::SpeechSynthesisEvent,
    /* mCreate */ SpeechSynthesisEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechSynthesisEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5307, // "IDBVersionChangeEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::IDBVersionChangeEvent,
    /* mCreate */ IDBVersionChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11325, // "Storage"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Storage,
    /* mCreate */ Storage_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4726, // "HTMLSourceElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLSourceElement,
    /* mCreate */ HTMLSourceElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6830, // "MutationObserver"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::MutationObserver,
    /* mCreate */ MutationObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3501, // "GeolocationPosition"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::GeolocationPosition,
    /* mCreate */ GeolocationPosition_Binding::CreateInterfaceObjects,
    /* mEnabled */ GeolocationPosition_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11503, // "SubmitEvent"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::SubmitEvent,
    /* mCreate */ SubmitEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9263, // "SVGDescElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGDescElement,
    /* mCreate */ SVGDescElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4547, // "HTMLOutputElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLOutputElement,
    /* mCreate */ HTMLOutputElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9131, // "SVGAnimatedTransformList"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SVGAnimatedTransformList,
    /* mCreate */ SVGAnimatedTransformList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8544, // "RTCPeerConnectionIceEvent"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::RTCPeerConnectionIceEvent,
    /* mCreate */ RTCPeerConnectionIceEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCPeerConnectionIceEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10611, // "SVGUseElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGUseElement,
    /* mCreate */ SVGUseElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5010, // "HTMLUnknownElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::HTMLUnknownElement,
    /* mCreate */ HTMLUnknownElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6084, // "MediaKeySession"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::MediaKeySession,
    /* mCreate */ MediaKeySession_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4294, // "HTMLLinkElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLLinkElement,
    /* mCreate */ HTMLLinkElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 622, // "BiquadFilterNode"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::BiquadFilterNode,
    /* mCreate */ BiquadFilterNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ BiquadFilterNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1351, // "ChildProcessMessageManager"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::ChildProcessMessageManager,
    /* mCreate */ ChildProcessMessageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChildProcessMessageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5417, // "ImageCaptureErrorEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::ImageCaptureErrorEvent,
    /* mCreate */ ImageCaptureErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ ImageCaptureErrorEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10240, // "SVGPolylineElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGPolylineElement,
    /* mCreate */ SVGPolylineElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 81, // "AddonEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::AddonEvent,
    /* mCreate */ AddonEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ AddonEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9665, // "SVGFEMergeElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEMergeElement,
    /* mCreate */ SVGFEMergeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12101, // "VRFrameData"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::VRFrameData,
    /* mCreate */ VRFrameData_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRFrameData_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7258, // "PaymentResponse"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::PaymentResponse,
    /* mCreate */ PaymentResponse_Binding::CreateInterfaceObjects,
    /* mEnabled */ PaymentResponse_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12912, // "XRFrame"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::XRFrame,
    /* mCreate */ XRFrame_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRFrame_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2345, // "ElementInternals"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::ElementInternals,
    /* mCreate */ ElementInternals_Binding::CreateInterfaceObjects,
    /* mEnabled */ ElementInternals_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6678, // "MozSharedMap"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MozSharedMap,
    /* mCreate */ MozSharedMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozSharedMap_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2180, // "DeviceOrientationEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::DeviceOrientationEvent,
    /* mCreate */ DeviceOrientationEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ DeviceOrientationEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3466, // "Geolocation"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::Geolocation,
    /* mCreate */ Geolocation_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11361, // "StreamFilter"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::StreamFilter,
    /* mCreate */ StreamFilter_Binding::CreateInterfaceObjects,
    /* mEnabled */ StreamFilter_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3910, // "HTMLDialogElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLDialogElement,
    /* mCreate */ HTMLDialogElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ HTMLDialogElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10869, // "ShadowRoot"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::ShadowRoot,
    /* mCreate */ ShadowRoot_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3891, // "HTMLDetailsElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::HTMLDetailsElement,
    /* mCreate */ HTMLDetailsElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7056, // "OscillatorNode"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::OscillatorNode,
    /* mCreate */ OscillatorNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ OscillatorNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12769, // "XMLHttpRequest"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XMLHttpRequest,
    /* mCreate */ XMLHttpRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ XMLHttpRequest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9097, // "SVGAnimatedRect"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGAnimatedRect,
    /* mCreate */ SVGAnimatedRect_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6640, // "MozDocumentObserver"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MozDocumentObserver,
    /* mCreate */ MozDocumentObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozDocumentObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10067, // "SVGMarkerElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGMarkerElement,
    /* mCreate */ SVGMarkerElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1025, // "CSSStyleSheet"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CSSStyleSheet,
    /* mCreate */ CSSStyleSheet_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8718, // "ReportBody"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::ReportBody,
    /* mCreate */ ReportBody_Binding::CreateInterfaceObjects,
    /* mEnabled */ ReportBody_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9821, // "SVGFETileElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGFETileElement,
    /* mCreate */ SVGFETileElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12435, // "WebGLQuery"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::WebGLQuery,
    /* mCreate */ WebGLQuery_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11250, // "SpeechSynthesisUtterance"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SpeechSynthesisUtterance,
    /* mCreate */ SpeechSynthesisUtterance_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechSynthesisUtterance_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3453, // "GamepadTouch"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::GamepadTouch,
    /* mCreate */ GamepadTouch_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadTouch_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8458, // "RTCDataChannel"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::RTCDataChannel,
    /* mCreate */ RTCDataChannel_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11989, // "UserInteraction"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::UserInteraction,
    /* mCreate */ UserInteraction_Binding::CreateInterfaceObjects,
    /* mEnabled */ UserInteraction_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7672, // "PlacesBookmarkAddition"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::PlacesBookmarkAddition,
    /* mCreate */ PlacesBookmarkAddition_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesBookmarkAddition_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8435, // "RTCDTMFToneChangeEvent"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::RTCDTMFToneChangeEvent,
    /* mCreate */ RTCDTMFToneChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13100, // "XRSpace"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::XRSpace,
    /* mCreate */ XRSpace_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRSpace_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13218, // "XULMenuElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XULMenuElement,
    /* mCreate */ XULMenuElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULMenuElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2224, // "Directory"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Directory,
    /* mCreate */ Directory_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6621, // "MozDocumentMatcher"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::MozDocumentMatcher,
    /* mCreate */ MozDocumentMatcher_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozDocumentMatcher_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9004, // "SVGAnimatedLengthList"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGAnimatedLengthList,
    /* mCreate */ SVGAnimatedLengthList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7597, // "PerformanceTiming"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::PerformanceTiming,
    /* mCreate */ PerformanceTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2012, // "DataTransfer"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::DataTransfer,
    /* mCreate */ DataTransfer_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3977, // "HTMLElement"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::HTMLElement,
    /* mCreate */ HTMLElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4760, // "HTMLStyleElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLStyleElement,
    /* mCreate */ HTMLStyleElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3181, // "GPUShaderStage"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::GPUShaderStage,
    /* mCreate */ GPUShaderStage_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUShaderStage_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4923, // "HTMLTextAreaElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLTextAreaElement,
    /* mCreate */ HTMLTextAreaElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4943, // "HTMLTimeElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLTimeElement,
    /* mCreate */ HTMLTimeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11736, // "TextTrackCue"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::TextTrackCue,
    /* mCreate */ TextTrackCue_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8705, // "Range"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::Range,
    /* mCreate */ Range_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11418, // "StructuredCloneTester"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::StructuredCloneTester,
    /* mCreate */ StructuredCloneTester_Binding::CreateInterfaceObjects,
    /* mEnabled */ StructuredCloneTester_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5885, // "MediaCapabilities"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::MediaCapabilities,
    /* mCreate */ MediaCapabilities_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaCapabilities_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7764, // "PlacesObservers"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::PlacesObservers,
    /* mCreate */ PlacesObservers_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesObservers_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5823, // "MIDIPort"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::MIDIPort,
    /* mCreate */ MIDIPort_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIPort_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4122, // "HTMLHRElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::HTMLHRElement,
    /* mCreate */ HTMLHRElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7794, // "PlacesVisit"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::PlacesVisit,
    /* mCreate */ PlacesVisit_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesVisit_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6885, // "NetworkInformation"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::NetworkInformation,
    /* mCreate */ NetworkInformation_Binding::CreateInterfaceObjects,
    /* mEnabled */ NetworkInformation_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4744, // "HTMLSpanElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLSpanElement,
    /* mCreate */ HTMLSpanElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12246, // "VideoTrackList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::VideoTrackList,
    /* mCreate */ VideoTrackList_Binding::CreateInterfaceObjects,
    /* mEnabled */ VideoTrackList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12548, // "WebGLTexture"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::WebGLTexture,
    /* mCreate */ WebGLTexture_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLTexture_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10167, // "SVGPathSegList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGPathSegList,
    /* mCreate */ SVGPathSegList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4276, // "HTMLLegendElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLLegendElement,
    /* mCreate */ HTMLLegendElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6359, // "MediaStreamTrack"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::MediaStreamTrack,
    /* mCreate */ MediaStreamTrack_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3928, // "HTMLDirectoryElement"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::HTMLDirectoryElement,
    /* mCreate */ HTMLDirectoryElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7132, // "ParentProcessMessageManager"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::ParentProcessMessageManager,
    /* mCreate */ ParentProcessMessageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ ParentProcessMessageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 433, // "AudioTrackList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::AudioTrackList,
    /* mCreate */ AudioTrackList_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioTrackList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4842, // "HTMLTableElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLTableElement,
    /* mCreate */ HTMLTableElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9568, // "SVGFEFuncBElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEFuncBElement,
    /* mCreate */ SVGFEFuncBElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 306, // "AudioDestinationNode"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::AudioDestinationNode,
    /* mCreate */ AudioDestinationNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioDestinationNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1487, // "ClipboardEvent"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::ClipboardEvent,
    /* mCreate */ ClipboardEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5855, // "MatchPatternSet"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::MatchPatternSet,
    /* mCreate */ MatchPatternSet_Binding::CreateInterfaceObjects,
    /* mEnabled */ MatchPatternSet_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 75, // "Addon"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::Addon,
    /* mCreate */ Addon_Binding::CreateInterfaceObjects,
    /* mEnabled */ Addon_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1437, // "ChromeNodeList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::ChromeNodeList,
    /* mCreate */ ChromeNodeList_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChromeNodeList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11951, // "UIEvent"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::UIEvent,
    /* mCreate */ UIEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3598, // "Grid"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Grid,
    /* mCreate */ Grid_Binding::CreateInterfaceObjects,
    /* mEnabled */ Grid_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 894, // "CSSMediaRule"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CSSMediaRule,
    /* mCreate */ CSSMediaRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4518, // "Option"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::HTMLOptionElement,
    /* mCreate */ HTMLOptionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3378, // "GamepadHapticActuator"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::GamepadHapticActuator,
    /* mCreate */ GamepadHapticActuator_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadHapticActuator_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12005, // "UserProximityEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::UserProximityEvent,
    /* mCreate */ UserProximityEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ UserProximityEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 271, // "AudioBufferSourceNode"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::AudioBufferSourceNode,
    /* mCreate */ AudioBufferSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioBufferSourceNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12749, // "Worklet"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Worklet,
    /* mCreate */ Worklet_Binding::CreateInterfaceObjects,
    /* mEnabled */ Worklet_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12694, // "WindowGlobalChild"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::WindowGlobalChild,
    /* mCreate */ WindowGlobalChild_Binding::CreateInterfaceObjects,
    /* mEnabled */ WindowGlobalChild_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10749, // "SecurityPolicyViolationEvent"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::SecurityPolicyViolationEvent,
    /* mCreate */ SecurityPolicyViolationEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11515, // "SubtleCrypto"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SubtleCrypto,
    /* mCreate */ SubtleCrypto_Binding::CreateInterfaceObjects,
    /* mEnabled */ SubtleCrypto_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9604, // "SVGFEFuncRElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEFuncRElement,
    /* mCreate */ SVGFEFuncRElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10330, // "SVGSVGElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGSVGElement,
    /* mCreate */ SVGSVGElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12628, // "WebSocket"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::WebSocket,
    /* mCreate */ WebSocket_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3220, // "GPUTextureUsage"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::GPUTextureUsage,
    /* mCreate */ GPUTextureUsage_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUTextureUsage_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8382, // "PushSubscriptionOptions"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::PushSubscriptionOptions,
    /* mCreate */ PushSubscriptionOptions_Binding::CreateInterfaceObjects,
    /* mEnabled */ PushSubscriptionOptions_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6862, // "NamedNodeMap"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::NamedNodeMap,
    /* mCreate */ NamedNodeMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6283, // "MediaStreamAudioDestinationNode"
    /* mNameLength */ 31,
    /* mConstructorId */ constructors::id::MediaStreamAudioDestinationNode,
    /* mCreate */ MediaStreamAudioDestinationNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaStreamAudioDestinationNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12888, // "XRBoundedReferenceSpace"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::XRBoundedReferenceSpace,
    /* mCreate */ XRBoundedReferenceSpace_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRBoundedReferenceSpace_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5161, // "IDBFileHandle"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::IDBFileHandle,
    /* mCreate */ IDBFileHandle_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 397, // "AudioScheduledSourceNode"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::AudioScheduledSourceNode,
    /* mCreate */ AudioScheduledSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13044, // "XRRenderState"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::XRRenderState,
    /* mCreate */ XRRenderState_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRRenderState_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9325, // "SVGFEColorMatrixElement"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SVGFEColorMatrixElement,
    /* mCreate */ SVGFEColorMatrixElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10390, // "SVGStringList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGStringList,
    /* mCreate */ SVGStringList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10598, // "SVGUnitTypes"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SVGUnitTypes,
    /* mCreate */ SVGUnitTypes_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5527, // "IntersectionObserver"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::IntersectionObserver,
    /* mCreate */ IntersectionObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ IntersectionObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10691, // "ScriptProcessorNode"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::ScriptProcessorNode,
    /* mCreate */ ScriptProcessorNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ ScriptProcessorNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11159, // "SpeechRecognitionResultList"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::SpeechRecognitionResultList,
    /* mCreate */ SpeechRecognitionResultList_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognitionResultList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12183, // "VTTCue"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::VTTCue,
    /* mCreate */ VTTCue_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10222, // "SVGPolygonElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGPolygonElement,
    /* mCreate */ SVGPolygonElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3009, // "GPUFence"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::GPUFence,
    /* mCreate */ GPUFence_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUFence_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 362, // "AudioParamMap"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::AudioParamMap,
    /* mCreate */ AudioParamMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioParamMap_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6536, // "MessageSender"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MessageSender,
    /* mCreate */ MessageSender_Binding::CreateInterfaceObjects,
    /* mEnabled */ MessageSender_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7695, // "PlacesBookmarkRemoved"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::PlacesBookmarkRemoved,
    /* mCreate */ PlacesBookmarkRemoved_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesBookmarkRemoved_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12845, // "XPathEvaluator"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::XPathEvaluator,
    /* mCreate */ XPathEvaluator_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5404, // "ImageCapture"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::ImageCapture,
    /* mCreate */ ImageCapture_Binding::CreateInterfaceObjects,
    /* mEnabled */ ImageCapture_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10051, // "SVGMPathElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGMPathElement,
    /* mCreate */ SVGMPathElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10912, // "SourceBuffer"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SourceBuffer,
    /* mCreate */ SourceBuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ SourceBuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7406, // "PerformanceMeasure"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::PerformanceMeasure,
    /* mCreate */ PerformanceMeasure_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1718, // "CredentialsContainer"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::CredentialsContainer,
    /* mCreate */ CredentialsContainer_Binding::CreateInterfaceObjects,
    /* mEnabled */ CredentialsContainer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9914, // "SVGGeometryElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGGeometryElement,
    /* mCreate */ SVGGeometryElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5450, // "ImageDocument"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ImageDocument,
    /* mCreate */ ImageDocument_Binding::CreateInterfaceObjects,
    /* mEnabled */ ImageDocument_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8986, // "SVGAnimatedLength"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGAnimatedLength,
    /* mCreate */ SVGAnimatedLength_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2830, // "GPUBindGroupLayout"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::GPUBindGroupLayout,
    /* mCreate */ GPUBindGroupLayout_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUBindGroupLayout_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12291, // "WebExtensionContentScript"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::WebExtensionContentScript,
    /* mCreate */ WebExtensionContentScript_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebExtensionContentScript_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2290, // "DominatorTree"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::DominatorTree,
    /* mCreate */ DominatorTree_Binding::CreateInterfaceObjects,
    /* mEnabled */ DominatorTree_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6315, // "MediaStreamAudioSourceNode"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::MediaStreamAudioSourceNode,
    /* mCreate */ MediaStreamAudioSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaStreamAudioSourceNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7875, // "PluginArray"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::PluginArray,
    /* mCreate */ PluginArray_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 604, // "BeforeUnloadEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::BeforeUnloadEvent,
    /* mCreate */ BeforeUnloadEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9212, // "SVGComponentTransferFunctionElement"
    /* mNameLength */ 35,
    /* mConstructorId */ constructors::id::SVGComponentTransferFunctionElement,
    /* mCreate */ SVGComponentTransferFunctionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6573, // "MouseEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::MouseEvent,
    /* mCreate */ MouseEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2084, // "DebuggerNotificationObserver"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::DebuggerNotificationObserver,
    /* mCreate */ DebuggerNotificationObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ DebuggerNotificationObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1464, // "ChromeWorker"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::ChromeWorker,
    /* mCreate */ ChromeWorker_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChromeWorker_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2905, // "GPUCommandBuffer"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::GPUCommandBuffer,
    /* mCreate */ GPUCommandBuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUCommandBuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1069, // "Cache"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::Cache,
    /* mCreate */ Cache_Binding::CreateInterfaceObjects,
    /* mEnabled */ Cache_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10778, // "Selection"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Selection,
    /* mCreate */ Selection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1756, // "CustomElementRegistry"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::CustomElementRegistry,
    /* mCreate */ CustomElementRegistry_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1502, // "ClipboardItem"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ClipboardItem,
    /* mCreate */ ClipboardItem_Binding::CreateInterfaceObjects,
    /* mEnabled */ ClipboardItem_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1837, // "DOMMatrix"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::DOMMatrix,
    /* mCreate */ DOMMatrix_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMMatrix_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8242, // "ProcessingInstruction"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::ProcessingInstruction,
    /* mCreate */ ProcessingInstruction_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 293, // "AudioContext"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::AudioContext,
    /* mCreate */ AudioContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7615, // "PeriodicWave"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::PeriodicWave,
    /* mCreate */ PeriodicWave_Binding::CreateInterfaceObjects,
    /* mEnabled */ PeriodicWave_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9176, // "SVGCircleElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGCircleElement,
    /* mCreate */ SVGCircleElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2063, // "DebuggerNotification"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::DebuggerNotification,
    /* mCreate */ DebuggerNotification_Binding::CreateInterfaceObjects,
    /* mEnabled */ DebuggerNotification_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8167, // "PresentationReceiver"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::PresentationReceiver,
    /* mCreate */ PresentationReceiver_Binding::CreateInterfaceObjects,
    /* mEnabled */ PresentationReceiver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7232, // "PaymentRequestUpdateEvent"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::PaymentRequestUpdateEvent,
    /* mCreate */ PaymentRequestUpdateEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PaymentRequestUpdateEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1599, // "ConstantSourceNode"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::ConstantSourceNode,
    /* mCreate */ ConstantSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ ConstantSourceNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10361, // "SVGSetElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGSetElement,
    /* mCreate */ SVGSetElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8493, // "RTCDtlsTransport"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::RTCDtlsTransport,
    /* mCreate */ RTCDtlsTransport_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCDtlsTransport_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2521, // "FileSystem"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::FileSystem,
    /* mCreate */ FileSystem_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6342, // "MediaStreamEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::MediaStreamEvent,
    /* mCreate */ MediaStreamEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaStreamEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7104, // "PaintRequestList"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PaintRequestList,
    /* mCreate */ PaintRequestList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10437, // "SVGSymbolElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGSymbolElement,
    /* mCreate */ SVGSymbolElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9401, // "SVGFEConvolveMatrixElement"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::SVGFEConvolveMatrixElement,
    /* mCreate */ SVGFEConvolveMatrixElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9289, // "SVGEllipseElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGEllipseElement,
    /* mCreate */ SVGEllipseElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10109, // "SVGMetadataElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGMetadataElement,
    /* mCreate */ SVGMetadataElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4993, // "HTMLUListElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLUListElement,
    /* mCreate */ HTMLUListElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6454, // "MessageBroadcaster"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::MessageBroadcaster,
    /* mCreate */ MessageBroadcaster_Binding::CreateInterfaceObjects,
    /* mEnabled */ MessageBroadcaster_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7071, // "PageTransitionEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::PageTransitionEvent,
    /* mCreate */ PageTransitionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12375, // "WebGLBuffer"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::WebGLBuffer,
    /* mCreate */ WebGLBuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLBuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6772, // "MozStorageStatementRow"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::MozStorageStatementRow,
    /* mCreate */ MozStorageStatementRow_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozStorageStatementRow_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11690, // "TextDecoder"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::TextDecoder,
    /* mCreate */ TextDecoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1055, // "CSSTransition"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CSSTransition,
    /* mCreate */ CSSTransition_Binding::CreateInterfaceObjects,
    /* mEnabled */ CSSTransition_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10209, // "SVGPointList"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SVGPointList,
    /* mCreate */ SVGPointList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13108, // "XRSystem"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::XRSystem,
    /* mCreate */ XRSystem_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRSystem_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 376, // "AudioProcessingEvent"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::AudioProcessingEvent,
    /* mCreate */ AudioProcessingEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioProcessingEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11546, // "TCPServerSocket"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::TCPServerSocket,
    /* mCreate */ TCPServerSocket_Binding::CreateInterfaceObjects,
    /* mEnabled */ TCPServerSocket_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7906, // "PointerEvent"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::PointerEvent,
    /* mCreate */ PointerEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12511, // "WebGLShaderPrecisionFormat"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::WebGLShaderPrecisionFormat,
    /* mCreate */ WebGLShaderPrecisionFormat_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLShaderPrecisionFormat_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 725, // "CSS2Properties"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::CSS2Properties,
    /* mCreate */ CSS2Properties_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1392, // "ChromeMessageBroadcaster"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::ChromeMessageBroadcaster,
    /* mCreate */ ChromeMessageBroadcaster_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChromeMessageBroadcaster_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2991, // "GPUDeviceLostInfo"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::GPUDeviceLostInfo,
    /* mCreate */ GPUDeviceLostInfo_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUDeviceLostInfo_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6795, // "MozWritableSharedMap"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::MozWritableSharedMap,
    /* mCreate */ MozWritableSharedMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozWritableSharedMap_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7293, // "PeerConnectionObserver"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::PeerConnectionObserver,
    /* mCreate */ PeerConnectionObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ PeerConnectionObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4879, // "HTMLTableSectionElement"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::HTMLTableSectionElement,
    /* mCreate */ HTMLTableSectionElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5119, // "IDBCursorWithValue"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::IDBCursorWithValue,
    /* mCreate */ IDBCursorWithValue_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5977, // "MediaDevices"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MediaDevices,
    /* mCreate */ MediaDevices_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaDevices_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11828, // "TrackEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TrackEvent,
    /* mCreate */ TrackEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 671, // "BrowsingContext"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::BrowsingContext,
    /* mCreate */ BrowsingContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ BrowsingContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1167, // "CanvasGradient"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::CanvasGradient,
    /* mCreate */ CanvasGradient_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10182, // "SVGPatternElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGPatternElement,
    /* mCreate */ SVGPatternElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3603, // "GridArea"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::GridArea,
    /* mCreate */ GridArea_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridArea_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3092, // "GPURenderBundleEncoder"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::GPURenderBundleEncoder,
    /* mCreate */ GPURenderBundleEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPURenderBundleEncoder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4171, // "HTMLHtmlElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLHtmlElement,
    /* mCreate */ HTMLHtmlElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3365, // "GamepadEvent"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::GamepadEvent,
    /* mCreate */ GamepadEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3196, // "GPUSwapChain"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::GPUSwapChain,
    /* mCreate */ GPUSwapChain_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUSwapChain_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10307, // "SVGRect"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::SVGRect,
    /* mCreate */ SVGRect_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11275, // "SpeechSynthesisVoice"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::SpeechSynthesisVoice,
    /* mCreate */ SpeechSynthesisVoice_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechSynthesisVoice_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 461, // "AudioWorkletNode"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::AudioWorkletNode,
    /* mCreate */ AudioWorkletNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioWorkletNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9586, // "SVGFEFuncGElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEFuncGElement,
    /* mCreate */ SVGFEFuncGElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4259, // "HTMLLabelElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLLabelElement,
    /* mCreate */ HTMLLabelElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11973, // "URLSearchParams"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::URLSearchParams,
    /* mCreate */ URLSearchParams_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11790, // "TimeRanges"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TimeRanges,
    /* mCreate */ TimeRanges_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11882, // "TreeColumns"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::TreeColumns,
    /* mCreate */ TreeColumns_Binding::CreateInterfaceObjects,
    /* mEnabled */ TreeColumns_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6408, // "MediaStreamTrackEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::MediaStreamTrackEvent,
    /* mCreate */ MediaStreamTrackEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2790, // "FrameLoader"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::FrameLoader,
    /* mCreate */ FrameLoader_Binding::CreateInterfaceObjects,
    /* mEnabled */ FrameLoader_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13124, // "XRViewerPose"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::XRViewerPose,
    /* mCreate */ XRViewerPose_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRViewerPose_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3478, // "GeolocationCoordinates"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::GeolocationCoordinates,
    /* mCreate */ GeolocationCoordinates_Binding::CreateInterfaceObjects,
    /* mEnabled */ GeolocationCoordinates_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7842, // "PlacesWeakCallbackWrapper"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::PlacesWeakCallbackWrapper,
    /* mCreate */ PlacesWeakCallbackWrapper_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesWeakCallbackWrapper_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10344, // "SVGScriptElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGScriptElement,
    /* mCreate */ SVGScriptElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4377, // "HTMLMenuItemElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLMenuItemElement,
    /* mCreate */ HTMLMenuItemElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ HTMLMenuItemElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5990, // "MediaElementAudioSourceNode"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::MediaElementAudioSourceNode,
    /* mCreate */ MediaElementAudioSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaElementAudioSourceNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11801, // "Touch"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::Touch,
    /* mCreate */ Touch_Binding::CreateInterfaceObjects,
    /* mEnabled */ Touch_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2449, // "FeaturePolicyViolationReportBody"
    /* mNameLength */ 32,
    /* mConstructorId */ constructors::id::FeaturePolicyViolationReportBody,
    /* mCreate */ FeaturePolicyViolationReportBody_Binding::CreateInterfaceObjects,
    /* mEnabled */ FeaturePolicyViolationReportBody_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3949, // "HTMLDivElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::HTMLDivElement,
    /* mCreate */ HTMLDivElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10492, // "SVGTextElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGTextElement,
    /* mCreate */ SVGTextElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3771, // "HTMLBodyElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLBodyElement,
    /* mCreate */ HTMLBodyElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3275, // "GPUValidationError"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::GPUValidationError,
    /* mCreate */ GPUValidationError_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUValidationError_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10200, // "SVGPoint"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::SVGPoint,
    /* mCreate */ SVGPoint_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7657, // "PlacesBookmark"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::PlacesBookmark,
    /* mCreate */ PlacesBookmark_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesBookmark_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 542, // "AuthenticatorResponse"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::AuthenticatorResponse,
    /* mCreate */ AuthenticatorResponse_Binding::CreateInterfaceObjects,
    /* mEnabled */ AuthenticatorResponse_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5211, // "IDBLocaleAwareKeyRange"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::IDBLocaleAwareKeyRange,
    /* mCreate */ IDBLocaleAwareKeyRange_Binding::CreateInterfaceObjects,
    /* mEnabled */ IDBLocaleAwareKeyRange_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11714, // "TextMetrics"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::TextMetrics,
    /* mCreate */ TextMetrics_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5264, // "IDBOpenDBRequest"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::IDBOpenDBRequest,
    /* mCreate */ IDBOpenDBRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5809, // "MIDIOutputMap"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MIDIOutputMap,
    /* mCreate */ MIDIOutputMap_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIOutputMap_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10420, // "SVGSwitchElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGSwitchElement,
    /* mCreate */ SVGSwitchElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8729, // "ReportingObserver"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::ReportingObserver,
    /* mCreate */ ReportingObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ ReportingObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2314, // "DynamicsCompressorNode"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::DynamicsCompressorNode,
    /* mCreate */ DynamicsCompressorNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ DynamicsCompressorNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11346, // "StorageManager"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::StorageManager,
    /* mCreate */ StorageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ StorageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8365, // "PushSubscription"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PushSubscription,
    /* mCreate */ PushSubscription_Binding::CreateInterfaceObjects,
    /* mEnabled */ PushSubscription_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10568, // "SVGTransform"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::SVGTransform,
    /* mCreate */ SVGTransform_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9987, // "SVGLength"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::SVGLength,
    /* mCreate */ SVGLength_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12731, // "WindowRoot"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::WindowRoot,
    /* mCreate */ WindowRoot_Binding::CreateInterfaceObjects,
    /* mEnabled */ WindowRoot_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7628, // "PermissionStatus"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PermissionStatus,
    /* mCreate */ PermissionStatus_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4430, // "HTMLModElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::HTMLModElement,
    /* mCreate */ HTMLModElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11374, // "StreamFilterDataEvent"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::StreamFilterDataEvent,
    /* mCreate */ StreamFilterDataEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ StreamFilterDataEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3067, // "GPUQueue"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::GPUQueue,
    /* mCreate */ GPUQueue_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUQueue_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5842, // "MatchPattern"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MatchPattern,
    /* mCreate */ MatchPattern_Binding::CreateInterfaceObjects,
    /* mEnabled */ MatchPattern_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7274, // "PeerConnectionImpl"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::PeerConnectionImpl,
    /* mCreate */ PeerConnectionImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ PeerConnectionImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3823, // "HTMLCollection"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::HTMLCollection,
    /* mCreate */ HTMLCollection_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11089, // "SpeechRecognitionError"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SpeechRecognitionError,
    /* mCreate */ SpeechRecognitionError_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognitionError_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3018, // "GPUMapMode"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::GPUMapMode,
    /* mCreate */ GPUMapMode_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUMapMode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4462, // "HTMLObjectElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLObjectElement,
    /* mCreate */ HTMLObjectElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7367, // "PerformanceEventTiming"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::PerformanceEventTiming,
    /* mCreate */ PerformanceEventTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceEventTiming_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6259, // "MediaSource"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::MediaSource,
    /* mCreate */ MediaSource_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaSource_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12784, // "XMLHttpRequestEventTarget"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::XMLHttpRequestEventTarget,
    /* mCreate */ XMLHttpRequestEventTarget_Binding::CreateInterfaceObjects,
    /* mEnabled */ XMLHttpRequestEventTarget_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 654, // "BroadcastChannel"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::BroadcastChannel,
    /* mCreate */ BroadcastChannel_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10994, // "webkitSpeechGrammarList"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SpeechGrammarList,
    /* mCreate */ SpeechGrammarList_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechGrammarList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7991, // "PrecompiledScript"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::PrecompiledScript,
    /* mCreate */ PrecompiledScript_Binding::CreateInterfaceObjects,
    /* mEnabled */ PrecompiledScript_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3165, // "GPUShaderModule"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::GPUShaderModule,
    /* mCreate */ GPUShaderModule_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUShaderModule_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4325, // "HTMLMarqueeElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::HTMLMarqueeElement,
    /* mCreate */ HTMLMarqueeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7177, // "PaymentAddress"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::PaymentAddress,
    /* mCreate */ PaymentAddress_Binding::CreateInterfaceObjects,
    /* mEnabled */ PaymentAddress_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13137, // "XRViewport"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::XRViewport,
    /* mCreate */ XRViewport_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRViewport_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1972, // "DOMStringList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::DOMStringList,
    /* mCreate */ DOMStringList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11333, // "StorageEvent"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::StorageEvent,
    /* mCreate */ StorageEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1933, // "DOMRectList"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::DOMRectList,
    /* mCreate */ DOMRectList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8570, // "RTCPeerConnectionStatic"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::RTCPeerConnectionStatic,
    /* mCreate */ RTCPeerConnectionStatic_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCPeerConnectionStatic_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1790, // "DOMException"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::DOMException,
    /* mCreate */ DOMException_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12261, // "VisualViewport"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::VisualViewport,
    /* mCreate */ VisualViewport_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2304, // "DragEvent"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::DragEvent,
    /* mCreate */ DragEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 687, // "BrowsingContextGroup"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::BrowsingContextGroup,
    /* mCreate */ BrowsingContextGroup_Binding::CreateInterfaceObjects,
    /* mEnabled */ BrowsingContextGroup_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5062, // "Headers"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Headers,
    /* mCreate */ Headers_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3805, // "HTMLCanvasElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLCanvasElement,
    /* mCreate */ HTMLCanvasElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11894, // "TreeContentView"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::TreeContentView,
    /* mCreate */ TreeContentView_Binding::CreateInterfaceObjects,
    /* mEnabled */ TreeContentView_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2362, // "ErrorEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::ErrorEvent,
    /* mCreate */ ErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12464, // "WebGLRenderingContext"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::WebGLRenderingContext,
    /* mCreate */ WebGLRenderingContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLRenderingContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12087, // "VRFieldOfView"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::VRFieldOfView,
    /* mCreate */ VRFieldOfView_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRFieldOfView_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2510, // "FileReader"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::FileReader,
    /* mCreate */ FileReader_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8317, // "PublicKeyCredential"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::PublicKeyCredential,
    /* mCreate */ PublicKeyCredential_Binding::CreateInterfaceObjects,
    /* mEnabled */ PublicKeyCredential_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5961, // "MediaDeviceInfo"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::MediaDeviceInfo,
    /* mCreate */ MediaDeviceInfo_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaDeviceInfo_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6473, // "MessageChannel"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::MessageChannel,
    /* mCreate */ MessageChannel_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12056, // "VRDisplayEvent"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::VRDisplayEvent,
    /* mCreate */ VRDisplayEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRDisplayEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2849, // "GPUBuffer"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::GPUBuffer,
    /* mCreate */ GPUBuffer_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUBuffer_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11702, // "TextEncoder"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::TextEncoder,
    /* mCreate */ TextEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8944, // "SVGAnimatedEnumeration"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGAnimatedEnumeration,
    /* mCreate */ SVGAnimatedEnumeration_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12860, // "XPathExpression"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::XPathExpression,
    /* mCreate */ XPathExpression_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10925, // "SourceBufferList"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SourceBufferList,
    /* mCreate */ SourceBufferList_Binding::CreateInterfaceObjects,
    /* mEnabled */ SourceBufferList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5190, // "IDBIndex"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::IDBIndex,
    /* mCreate */ IDBIndex_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5281, // "IDBRequest"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::IDBRequest,
    /* mCreate */ IDBRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9838, // "SVGFETurbulenceElement"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGFETurbulenceElement,
    /* mCreate */ SVGFETurbulenceElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5464, // "InputEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::InputEvent,
    /* mCreate */ InputEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12151, // "VRServiceTest"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::VRServiceTest,
    /* mCreate */ VRServiceTest_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRServiceTest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4397, // "HTMLMetaElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLMetaElement,
    /* mCreate */ HTMLMetaElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6691, // "MozSharedMapChangeEvent"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::MozSharedMapChangeEvent,
    /* mCreate */ MozSharedMapChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozSharedMapChangeEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12953, // "XRInputSourceEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::XRInputSourceEvent,
    /* mCreate */ XRInputSourceEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRInputSourceEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2940, // "GPUComputePassEncoder"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::GPUComputePassEncoder,
    /* mCreate */ GPUComputePassEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUComputePassEncoder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13327, // "mozRTCSessionDescription"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::mozRTCSessionDescription,
    /* mCreate */ mozRTCSessionDescription_Binding::CreateInterfaceObjects,
    /* mEnabled */ mozRTCSessionDescription_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1891, // "DOMPoint"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::DOMPoint,
    /* mCreate */ DOMPoint_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMPoint_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10138, // "SVGNumberList"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::SVGNumberList,
    /* mCreate */ SVGNumberList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9307, // "SVGFEBlendElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEBlendElement,
    /* mCreate */ SVGFEBlendElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6246, // "MediaSession"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MediaSession,
    /* mCreate */ MediaSession_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaSession_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12584, // "WebGLUniformLocation"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::WebGLUniformLocation,
    /* mCreate */ WebGLUniformLocation_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLUniformLocation_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10404, // "SVGStyleElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGStyleElement,
    /* mCreate */ SVGStyleElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7217, // "PaymentRequest"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::PaymentRequest,
    /* mCreate */ PaymentRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ PaymentRequest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8473, // "RTCDataChannelEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::RTCDataChannelEvent,
    /* mCreate */ RTCDataChannelEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCDataChannelEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1182, // "CanvasPattern"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::CanvasPattern,
    /* mCreate */ CanvasPattern_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11308, // "StereoPannerNode"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::StereoPannerNode,
    /* mCreate */ StereoPannerNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ StereoPannerNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3332, // "GamepadButton"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::GamepadButton,
    /* mCreate */ GamepadButton_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadButton_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9647, // "SVGFEImageElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEImageElement,
    /* mCreate */ SVGFEImageElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6488, // "MessageEvent"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::MessageEvent,
    /* mCreate */ MessageEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12742, // "Worker"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Worker,
    /* mCreate */ Worker_Binding::CreateInterfaceObjects,
    /* mEnabled */ Worker_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10259, // "SVGPreserveAspectRatio"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGPreserveAspectRatio,
    /* mCreate */ SVGPreserveAspectRatio_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2496, // "File"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::File,
    /* mCreate */ File_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3645, // "GridTrack"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::GridTrack,
    /* mCreate */ GridTrack_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridTrack_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1088, // "CallbackDebuggerNotification"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::CallbackDebuggerNotification,
    /* mCreate */ CallbackDebuggerNotification_Binding::CreateInterfaceObjects,
    /* mEnabled */ CallbackDebuggerNotification_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12662, // "WheelEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::WheelEvent,
    /* mCreate */ WheelEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 327, // "AudioListener"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::AudioListener,
    /* mCreate */ AudioListener_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioListener_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10282, // "SVGRadialGradientElement"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SVGRadialGradientElement,
    /* mCreate */ SVGRadialGradientElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7167, // "PathUtils"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::PathUtils,
    /* mCreate */ PathUtils_Binding::CreateInterfaceObjects,
    /* mEnabled */ PathUtils_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10976, // "SpeechGrammarList"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SpeechGrammarList,
    /* mCreate */ SpeechGrammarList_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechGrammarList_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12113, // "VRMockController"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::VRMockController,
    /* mCreate */ VRMockController_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRMockController_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5475, // "InspectorFontFace"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::InspectorFontFace,
    /* mCreate */ InspectorFontFace_Binding::CreateInterfaceObjects,
    /* mEnabled */ InspectorFontFace_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3612, // "GridDimension"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::GridDimension,
    /* mCreate */ GridDimension_Binding::CreateInterfaceObjects,
    /* mEnabled */ GridDimension_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 955, // "CSSPseudoElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::CSSPseudoElement,
    /* mCreate */ CSSPseudoElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ CSSPseudoElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 564, // "BarProp"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::BarProp,
    /* mCreate */ BarProp_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4361, // "HTMLMenuElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLMenuElement,
    /* mCreate */ HTMLMenuElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2749, // "FormData"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::FormData,
    /* mCreate */ FormData_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9509, // "SVGFEDropShadowElement"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGFEDropShadowElement,
    /* mCreate */ SVGFEDropShadowElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11780, // "TimeEvent"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::TimeEvent,
    /* mCreate */ TimeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9532, // "SVGFEFloodElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGFEFloodElement,
    /* mCreate */ SVGFEFloodElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 740, // "CSSAnimation"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::CSSAnimation,
    /* mCreate */ CSSAnimation_Binding::CreateInterfaceObjects,
    /* mEnabled */ CSSAnimation_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10673, // "ScreenOrientation"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::ScreenOrientation,
    /* mCreate */ ScreenOrientation_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12387, // "WebGLContextEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::WebGLContextEvent,
    /* mCreate */ WebGLContextEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLContextEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2874, // "GPUCanvasContext"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::GPUCanvasContext,
    /* mCreate */ GPUCanvasContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUCanvasContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6847, // "MutationRecord"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::MutationRecord,
    /* mCreate */ MutationRecord_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7806, // "PlacesVisitRemoved"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::PlacesVisitRemoved,
    /* mCreate */ PlacesVisitRemoved_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesVisitRemoved_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5798, // "MIDIOutput"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::MIDIOutput,
    /* mCreate */ MIDIOutput_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIOutput_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11203, // "SpeechSynthesisErrorEvent"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::SpeechSynthesisErrorEvent,
    /* mCreate */ SpeechSynthesisErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechSynthesisErrorEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1961, // "DOMRequest"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::DOMRequest,
    /* mCreate */ DOMRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6715, // "MozStorageAsyncStatementParams"
    /* mNameLength */ 30,
    /* mConstructorId */ constructors::id::MozStorageAsyncStatementParams,
    /* mCreate */ MozStorageAsyncStatementParams_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozStorageAsyncStatementParams_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9484, // "SVGFEDistantLightElement"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SVGFEDistantLightElement,
    /* mCreate */ SVGFEDistantLightElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4152, // "HTMLHeadingElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::HTMLHeadingElement,
    /* mCreate */ HTMLHeadingElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5677, // "KeyframeEffect"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::KeyframeEffect,
    /* mCreate */ KeyframeEffect_Binding::CreateInterfaceObjects,
    /* mEnabled */ KeyframeEffect_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7390, // "PerformanceMark"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::PerformanceMark,
    /* mCreate */ PerformanceMark_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6601, // "MozCanvasPrintState"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MozCanvasPrintState,
    /* mCreate */ MozCanvasPrintState_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozCanvasPrintState_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1645, // "ContentProcessMessageManager"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::ContentProcessMessageManager,
    /* mCreate */ ContentProcessMessageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ ContentProcessMessageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9428, // "SVGFEDiffuseLightingElement"
    /* mNameLength */ 27,
    /* mConstructorId */ constructors::id::SVGFEDiffuseLightingElement,
    /* mCreate */ SVGFEDiffuseLightingElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9193, // "SVGClipPathElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGClipPathElement,
    /* mCreate */ SVGClipPathElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2707, // "FontFace"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::FontFace,
    /* mCreate */ FontFace_Binding::CreateInterfaceObjects,
    /* mEnabled */ FontFace_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9799, // "SVGFESpotLightElement"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGFESpotLightElement,
    /* mCreate */ SVGFESpotLightElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4859, // "HTMLTableRowElement"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::HTMLTableRowElement,
    /* mCreate */ HTMLTableRowElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7573, // "PerformanceServerTiming"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::PerformanceServerTiming,
    /* mCreate */ PerformanceServerTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ PerformanceServerTiming_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8908, // "SVGAnimatedAngle"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::SVGAnimatedAngle,
    /* mCreate */ SVGAnimatedAngle_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 341, // "AudioNode"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::AudioNode,
    /* mCreate */ AudioNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11060, // "SpeechRecognitionAlternative"
    /* mNameLength */ 28,
    /* mConstructorId */ constructors::id::SpeechRecognitionAlternative,
    /* mCreate */ SpeechRecognitionAlternative_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognitionAlternative_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4310, // "HTMLMapElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::HTMLMapElement,
    /* mCreate */ HTMLMapElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2654, // "FluentBundle"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::FluentBundle,
    /* mCreate */ FluentBundle_Binding::CreateInterfaceObjects,
    /* mEnabled */ FluentBundle_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 254, // "Attr"
    /* mNameLength */ 4,
    /* mConstructorId */ constructors::id::Attr,
    /* mCreate */ Attr_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1534, // "CloseEvent"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::CloseEvent,
    /* mCreate */ CloseEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7524, // "PerformancePaintTiming"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::PerformancePaintTiming,
    /* mCreate */ PerformancePaintTiming_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2859, // "GPUBufferUsage"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::GPUBufferUsage,
    /* mCreate */ GPUBufferUsage_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUBufferUsage_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11296, // "StaticRange"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::StaticRange,
    /* mCreate */ StaticRange_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9026, // "SVGAnimatedNumber"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGAnimatedNumber,
    /* mCreate */ SVGAnimatedNumber_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 448, // "AudioWorklet"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::AudioWorklet,
    /* mCreate */ AudioWorklet_Binding::CreateInterfaceObjects,
    /* mEnabled */ AudioWorklet_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3855, // "HTMLDataElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HTMLDataElement,
    /* mCreate */ HTMLDataElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9952, // "SVGGraphicsElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGGraphicsElement,
    /* mCreate */ SVGGraphicsElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4222, // "Image"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::HTMLImageElement,
    /* mCreate */ HTMLImageElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1325, // "CheckerboardReportService"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::CheckerboardReportService,
    /* mCreate */ CheckerboardReportService_Binding::CreateInterfaceObjects,
    /* mEnabled */ CheckerboardReportService_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11135, // "SpeechRecognitionResult"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::SpeechRecognitionResult,
    /* mCreate */ SpeechRecognitionResult_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechRecognitionResult_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8264, // "ProgressEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ProgressEvent,
    /* mCreate */ ProgressEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5781, // "MIDIMessageEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::MIDIMessageEvent,
    /* mCreate */ MIDIMessageEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIMessageEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 7972, // "PositionStateEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::PositionStateEvent,
    /* mCreate */ PositionStateEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ PositionStateEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4692, // "HTMLSelectElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLSelectElement,
    /* mCreate */ HTMLSelectElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 3521, // "GeolocationPositionError"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::GeolocationPositionError,
    /* mCreate */ GeolocationPositionError_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10152, // "SVGPathElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGPathElement,
    /* mCreate */ SVGPathElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 572, // "BaseAudioContext"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::BaseAudioContext,
    /* mCreate */ BaseAudioContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1917, // "DOMQuad"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::DOMQuad,
    /* mCreate */ DOMQuad_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMQuad_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12712, // "WindowGlobalParent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::WindowGlobalParent,
    /* mCreate */ WindowGlobalParent_Binding::CreateInterfaceObjects,
    /* mEnabled */ WindowGlobalParent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6550, // "MimeType"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::MimeType,
    /* mCreate */ MimeType_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6746, // "MozStorageStatementParams"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::MozStorageStatementParams,
    /* mCreate */ MozStorageStatementParams_Binding::CreateInterfaceObjects,
    /* mEnabled */ MozStorageStatementParams_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11679, // "TextClause"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TextClause,
    /* mCreate */ TextClause_Binding::CreateInterfaceObjects,
    /* mEnabled */ TextClause_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 105, // "AddonManager"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::AddonManager,
    /* mCreate */ AddonManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ AddonManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1925, // "DOMRect"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::DOMRect,
    /* mCreate */ DOMRect_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2802, // "GPU"
    /* mNameLength */ 3,
    /* mConstructorId */ constructors::id::GPU,
    /* mCreate */ GPU_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPU_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12680, // "WindowContext"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::WindowContext,
    /* mCreate */ WindowContext_Binding::CreateInterfaceObjects,
    /* mEnabled */ WindowContext_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10788, // "ServiceWorker"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::ServiceWorker,
    /* mCreate */ ServiceWorker_Binding::CreateInterfaceObjects,
    /* mEnabled */ ServiceWorker_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6149, // "MediaList"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::MediaList,
    /* mCreate */ MediaList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1618, // "ContentFrameMessageManager"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::ContentFrameMessageManager,
    /* mCreate */ ContentFrameMessageManager_Binding::CreateInterfaceObjects,
    /* mEnabled */ ContentFrameMessageManager_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13191, // "XULElement"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::XULElement,
    /* mCreate */ XULElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5109, // "IDBCursor"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::IDBCursor,
    /* mCreate */ IDBCursor_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9728, // "SVGFEOffsetElement"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGFEOffsetElement,
    /* mCreate */ SVGFEOffsetElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7091, // "PaintRequest"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::PaintRequest,
    /* mCreate */ PaintRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8526, // "RTCPeerConnection"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::RTCPeerConnection,
    /* mCreate */ RTCPeerConnection_Binding::CreateInterfaceObjects,
    /* mEnabled */ RTCPeerConnection_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6875, // "Navigator"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::Navigator,
    /* mCreate */ Navigator_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6271, // "MediaStream"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::MediaStream,
    /* mCreate */ MediaStream_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 7825, // "PlacesVisitTitle"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PlacesVisitTitle,
    /* mCreate */ PlacesVisitTitle_Binding::CreateInterfaceObjects,
    /* mEnabled */ PlacesVisitTitle_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1142, // "CanvasCaptureMediaStream"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::CanvasCaptureMediaStream,
    /* mCreate */ CanvasCaptureMediaStream_Binding::CreateInterfaceObjects,
    /* mEnabled */ CanvasCaptureMediaStream_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3346, // "GamepadButtonEvent"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::GamepadButtonEvent,
    /* mCreate */ GamepadButtonEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ GamepadButtonEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5574, // "JSProcessActorChild"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::JSProcessActorChild,
    /* mCreate */ JSProcessActorChild_Binding::CreateInterfaceObjects,
    /* mEnabled */ JSProcessActorChild_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5692, // "L10nOverlays"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::L10nOverlays,
    /* mCreate */ L10nOverlays_Binding::CreateInterfaceObjects,
    /* mEnabled */ L10nOverlays_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8349, // "PushManagerImpl"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::PushManagerImpl,
    /* mCreate */ PushManagerImpl_Binding::CreateInterfaceObjects,
    /* mEnabled */ PushManagerImpl_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8278, // "PromiseDebugging"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::PromiseDebugging,
    /* mCreate */ PromiseDebugging_Binding::CreateInterfaceObjects,
    /* mEnabled */ PromiseDebugging_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3294, // "GainNode"
    /* mNameLength */ 8,
    /* mConstructorId */ constructors::id::GainNode,
    /* mCreate */ GainNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ GainNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6018, // "MediaEncryptedEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MediaEncryptedEvent,
    /* mCreate */ MediaEncryptedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9683, // "SVGFEMergeNodeElement"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::SVGFEMergeNodeElement,
    /* mCreate */ SVGFEMergeNodeElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4413, // "HTMLMeterElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLMeterElement,
    /* mCreate */ HTMLMeterElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11440, // "StyleSheet"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::StyleSheet,
    /* mCreate */ StyleSheet_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5329, // "IIRFilterNode"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::IIRFilterNode,
    /* mCreate */ IIRFilterNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ IIRFilterNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 509, // "AuthenticatorAttestationResponse"
    /* mNameLength */ 32,
    /* mConstructorId */ constructors::id::AuthenticatorAttestationResponse,
    /* mCreate */ AuthenticatorAttestationResponse_Binding::CreateInterfaceObjects,
    /* mEnabled */ AuthenticatorAttestationResponse_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13306, // "mozRTCPeerConnection"
    /* mNameLength */ 20,
    /* mConstructorId */ constructors::id::mozRTCPeerConnection,
    /* mCreate */ mozRTCPeerConnection_Binding::CreateInterfaceObjects,
    /* mEnabled */ mozRTCPeerConnection_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11910, // "TreeWalker"
    /* mNameLength */ 10,
    /* mConstructorId */ constructors::id::TreeWalker,
    /* mCreate */ TreeWalker_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5758, // "MIDIInput"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::MIDIInput,
    /* mCreate */ MIDIInput_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIInput_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5548, // "IntersectionObserverEntry"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::IntersectionObserverEntry,
    /* mCreate */ IntersectionObserverEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ IntersectionObserverEntry_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3787, // "HTMLButtonElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLButtonElement,
    /* mCreate */ HTMLButtonElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 943, // "CSSPageRule"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::CSSPageRule,
    /* mCreate */ CSSPageRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11187, // "SpeechSynthesis"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SpeechSynthesis,
    /* mCreate */ SpeechSynthesis_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechSynthesis_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8711, // "Report"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::Report,
    /* mCreate */ Report_Binding::CreateInterfaceObjects,
    /* mEnabled */ Report_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 1863, // "DOMMatrixReadOnly"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::DOMMatrixReadOnly,
    /* mCreate */ DOMMatrixReadOnly_Binding::CreateInterfaceObjects,
    /* mEnabled */ DOMMatrixReadOnly_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10956, // "webkitSpeechGrammar"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::SpeechGrammar,
    /* mCreate */ SpeechGrammar_Binding::CreateInterfaceObjects,
    /* mEnabled */ SpeechGrammar_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2772, // "FrameCrashedEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::FrameCrashedEvent,
    /* mCreate */ FrameCrashedEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ FrameCrashedEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5343, // "IOUtils"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::IOUtils,
    /* mCreate */ IOUtils_Binding::CreateInterfaceObjects,
    /* mEnabled */ IOUtils_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12605, // "WebGLVertexArrayObject"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::WebGLVertexArrayObject,
    /* mCreate */ WebGLVertexArrayObject_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2373, // "Event"
    /* mNameLength */ 5,
    /* mConstructorId */ constructors::id::Event,
    /* mCreate */ Event_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6222, // "MediaRecorderErrorEvent"
    /* mNameLength */ 23,
    /* mConstructorId */ constructors::id::MediaRecorderErrorEvent,
    /* mCreate */ MediaRecorderErrorEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12934, // "XRInputSourceArray"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::XRInputSourceArray,
    /* mCreate */ XRInputSourceArray_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRInputSourceArray_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9747, // "SVGFEPointLightElement"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::SVGFEPointLightElement,
    /* mCreate */ SVGFEPointLightElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6208, // "MediaRecorder"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MediaRecorder,
    /* mCreate */ MediaRecorder_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8818, // "SVGAElement"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::SVGAElement,
    /* mCreate */ SVGAElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6173, // "MediaQueryList"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::MediaQueryList,
    /* mCreate */ MediaQueryList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9622, // "SVGFEGaussianBlurElement"
    /* mNameLength */ 24,
    /* mConstructorId */ constructors::id::SVGFEGaussianBlurElement,
    /* mCreate */ SVGFEGaussianBlurElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 9349, // "SVGFEComponentTransferElement"
    /* mNameLength */ 29,
    /* mConstructorId */ constructors::id::SVGFEComponentTransferElement,
    /* mCreate */ SVGFEComponentTransferElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4976, // "HTMLTrackElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLTrackElement,
    /* mCreate */ HTMLTrackElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8747, // "Request"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Request,
    /* mCreate */ Request_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12972, // "XRInputSourcesChangeEvent"
    /* mNameLength */ 25,
    /* mConstructorId */ constructors::id::XRInputSourcesChangeEvent,
    /* mCreate */ XRInputSourcesChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRInputSourcesChangeEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2162, // "DeviceMotionEvent"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::DeviceMotionEvent,
    /* mCreate */ DeviceMotionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ DeviceMotionEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12998, // "XRPose"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::XRPose,
    /* mCreate */ XRPose_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRPose_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 926, // "CSSNamespaceRule"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::CSSNamespaceRule,
    /* mCreate */ CSSNamespaceRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1945, // "DOMRectReadOnly"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::DOMRectReadOnly,
    /* mCreate */ DOMRectReadOnly_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2716, // "FontFaceSet"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::FontFaceSet,
    /* mCreate */ FontFaceSet_Binding::CreateInterfaceObjects,
    /* mEnabled */ FontFaceSet_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3838, // "HTMLDListElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLDListElement,
    /* mCreate */ HTMLDListElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8790, // "ResizeObserverSize"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::ResizeObserverSize,
    /* mCreate */ ResizeObserverSize_Binding::CreateInterfaceObjects,
    /* mEnabled */ ResizeObserverSize_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10625, // "SVGViewElement"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::SVGViewElement,
    /* mCreate */ SVGViewElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2113, // "DelayNode"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::DelayNode,
    /* mCreate */ DelayNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ DelayNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3236, // "GPUTextureView"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::GPUTextureView,
    /* mCreate */ GPUTextureView_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPUTextureView_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 8967, // "SVGAnimatedInteger"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::SVGAnimatedInteger,
    /* mCreate */ SVGAnimatedInteger_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5663, // "KeyboardEvent"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::KeyboardEvent,
    /* mCreate */ KeyboardEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 60, // "AccessibleNode"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::AccessibleNode,
    /* mCreate */ AccessibleNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AccessibleNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 142, // "AnalyserNode"
    /* mNameLength */ 12,
    /* mConstructorId */ constructors::id::AnalyserNode,
    /* mCreate */ AnalyserNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ AnalyserNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9113, // "SVGAnimatedString"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGAnimatedString,
    /* mCreate */ SVGAnimatedString_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 4959, // "HTMLTitleElement"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::HTMLTitleElement,
    /* mCreate */ HTMLTitleElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8208, // "PrioEncoder"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::PrioEncoder,
    /* mCreate */ PrioEncoder_Binding::CreateInterfaceObjects,
    /* mEnabled */ PrioEncoder_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 10851, // "SessionStoreUtils"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SessionStoreUtils,
    /* mCreate */ SessionStoreUtils_Binding::CreateInterfaceObjects,
    /* mEnabled */ SessionStoreUtils_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 11921, // "U2F"
    /* mNameLength */ 3,
    /* mConstructorId */ constructors::id::U2F,
    /* mCreate */ U2F_Binding::CreateInterfaceObjects,
    /* mEnabled */ U2F_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13233, // "XULPopupElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::XULPopupElement,
    /* mCreate */ XULPopupElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULPopupElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 9971, // "SVGImageElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::SVGImageElement,
    /* mCreate */ SVGImageElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8839, // "SVGAnimateElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::SVGAnimateElement,
    /* mCreate */ SVGAnimateElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 11959, // "URL"
    /* mNameLength */ 3,
    /* mConstructorId */ constructors::id::URL,
    /* mCreate */ URL_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 18, // "AbortController"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::AbortController,
    /* mCreate */ AbortController_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 46, // "AbstractRange"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::AbstractRange,
    /* mCreate */ AbstractRange_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 980, // "CSSRuleList"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::CSSRuleList,
    /* mCreate */ CSSRuleList_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1688, // "CreateOfferRequest"
    /* mNameLength */ 18,
    /* mConstructorId */ constructors::id::CreateOfferRequest,
    /* mCreate */ CreateOfferRequest_Binding::CreateInterfaceObjects,
    /* mEnabled */ CreateOfferRequest_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3076, // "GPURenderBundle"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::GPURenderBundle,
    /* mCreate */ GPURenderBundle_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPURenderBundle_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 237, // "AnonymousContent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::AnonymousContent,
    /* mCreate */ AnonymousContent_Binding::CreateInterfaceObjects,
    /* mEnabled */ AnonymousContent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5292, // "IDBTransaction"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::IDBTransaction,
    /* mCreate */ IDBTransaction_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 6139, // "MediaKeys"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::MediaKeys,
    /* mCreate */ MediaKeys_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13202, // "XULFrameElement"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::XULFrameElement,
    /* mCreate */ XULFrameElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ XULFrameElement_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 4674, // "HTMLScriptElement"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::HTMLScriptElement,
    /* mCreate */ HTMLScriptElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 219, // "AnimationTimeline"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::AnimationTimeline,
    /* mCreate */ AnimationTimeline_Binding::CreateInterfaceObjects,
    /* mEnabled */ AnimationTimeline_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 861, // "CSSKeyframeRule"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::CSSKeyframeRule,
    /* mCreate */ CSSKeyframeRule_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 1276, // "ChannelSplitterNode"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::ChannelSplitterNode,
    /* mCreate */ ChannelSplitterNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ ChannelSplitterNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 2599, // "FileSystemFileEntry"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::FileSystemFileEntry,
    /* mCreate */ FileSystemFileEntry_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 10128, // "SVGNumber"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::SVGNumber,
    /* mCreate */ SVGNumber_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5046, // "HashChangeEvent"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::HashChangeEvent,
    /* mCreate */ HashChangeEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 13075, // "XRSession"
    /* mNameLength */ 9,
    /* mConstructorId */ constructors::id::XRSession,
    /* mCreate */ XRSession_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRSession_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12034, // "VRDisplayCapabilities"
    /* mNameLength */ 21,
    /* mConstructorId */ constructors::id::VRDisplayCapabilities,
    /* mCreate */ VRDisplayCapabilities_Binding::CreateInterfaceObjects,
    /* mEnabled */ VRDisplayCapabilities_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 34, // "AbortSignal"
    /* mNameLength */ 11,
    /* mConstructorId */ constructors::id::AbortSignal,
    /* mCreate */ AbortSignal_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8881, // "SVGAnimateTransformElement"
    /* mNameLength */ 26,
    /* mConstructorId */ constructors::id::SVGAnimateTransformElement,
    /* mCreate */ SVGAnimateTransformElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 2624, // "FlexItemValues"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::FlexItemValues,
    /* mCreate */ FlexItemValues_Binding::CreateInterfaceObjects,
    /* mEnabled */ FlexItemValues_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 5945, // "MediaController"
    /* mNameLength */ 15,
    /* mConstructorId */ constructors::id::MediaController,
    /* mCreate */ MediaController_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaController_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6584, // "MouseScrollEvent"
    /* mNameLength */ 16,
    /* mConstructorId */ constructors::id::MouseScrollEvent,
    /* mCreate */ MouseScrollEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 12831, // "XMLSerializer"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::XMLSerializer,
    /* mCreate */ XMLSerializer_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 5738, // "MIDIConnectionEvent"
    /* mNameLength */ 19,
    /* mConstructorId */ constructors::id::MIDIConnectionEvent,
    /* mCreate */ MIDIConnectionEvent_Binding::CreateInterfaceObjects,
    /* mEnabled */ MIDIConnectionEvent_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3136, // "GPURenderPipeline"
    /* mNameLength */ 17,
    /* mConstructorId */ constructors::id::GPURenderPipeline,
    /* mCreate */ GPURenderPipeline_Binding::CreateInterfaceObjects,
    /* mEnabled */ GPURenderPipeline_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3303, // "Gamepad"
    /* mNameLength */ 7,
    /* mConstructorId */ constructors::id::Gamepad,
    /* mCreate */ Gamepad_Binding::CreateInterfaceObjects,
    /* mEnabled */ Gamepad_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6559, // "MimeTypeArray"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::MimeTypeArray,
    /* mCreate */ MimeTypeArray_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  },
  {
    /* mNameOffset */ 8755, // "ResizeObserver"
    /* mNameLength */ 14,
    /* mConstructorId */ constructors::id::ResizeObserver,
    /* mCreate */ ResizeObserver_Binding::CreateInterfaceObjects,
    /* mEnabled */ ResizeObserver_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 6376, // "MediaStreamTrackAudioSourceNode"
    /* mNameLength */ 31,
    /* mConstructorId */ constructors::id::MediaStreamTrackAudioSourceNode,
    /* mCreate */ MediaStreamTrackAudioSourceNode_Binding::CreateInterfaceObjects,
    /* mEnabled */ MediaStreamTrackAudioSourceNode_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 12561, // "WebGLTransformFeedback"
    /* mNameLength */ 22,
    /* mConstructorId */ constructors::id::WebGLTransformFeedback,
    /* mCreate */ WebGLTransformFeedback_Binding::CreateInterfaceObjects,
    /* mEnabled */ WebGLTransformFeedback_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 13117, // "XRView"
    /* mNameLength */ 6,
    /* mConstructorId */ constructors::id::XRView,
    /* mCreate */ XRView_Binding::CreateInterfaceObjects,
    /* mEnabled */ XRView_Binding::ConstructorEnabled
  },
  {
    /* mNameOffset */ 3741, // "HTMLBRElement"
    /* mNameLength */ 13,
    /* mConstructorId */ constructors::id::HTMLBRElement,
    /* mCreate */ HTMLBRElement_Binding::CreateInterfaceObjects,
    /* mEnabled */ nullptr
  }
};


const WebIDLNameTableEntry*
WebIDLGlobalNameHash::GetEntry(JSLinearString* aKey)
{
  static const uint16_t BASES[] = {
       1,  17,   0,   2,   6,  62,  11,   8,  29,   2,  36,  12,  22,  43, 161,   3,
      14,  14,  30,   3,  38,   1,  63,   2,   1,   2,   0,  72,  12,  11,  14,   0,
      25,   5,  13,   3,   4,   5,  23,  31,   2,  55,  12,   1,  24,  73,  11,  19,
      16,  15,  13,   7,   8,  15,  90,  93,  19,   1,   8,  32, 126,   1,   1,  62,
       2,   3,  42,  21,  25, 215,  12,  66,  36,  18,   4,  11,   8,   7,   1,  36,
       2,   9, 121,  44,  16,  14,  13,  46,   4,   2,  91,   0,   4,  46,   1,  15,
      16,  16,  76,  10,  96,   4,  80,  16,  39,  20,  42,   5,  21,   8,  97,  70,
      21,   6,  69,   0,  20,  40,  16,  38,   9,   3,   5,   1,   3,  51,  20,  37,
       0,   2,  77,   5,   1,  20,  24,  34,   1, 319,   2,   2,   4,  55,  27,   2,
      45,  17,  14,  32, 143,  68,   0,   1,  75,   8,   1,  16,  88,   1,  34,   7,
       1,  15,  73,  13,   1,   8,  24,  14,  82,   1, 107,  27,  54,  49,   0,   8,
      10,  17,   0,  24,  12,   7,   9, 303,  87, 173,   7,  58,  63,  23,  28,   6,
       0,  18,  86,   0, 104,   1,   1,  30,  58, 163, 188,  30,   8,   5, 113,  14,
       0, 270,  10,  18,   2, 394, 218,   8,   8,  15, 479,  82, 179,  14,  20,   9,
       8, 302, 115, 343, 389,  36,   4,   1,  20,   2,  77,  14,   2,  84, 185,  70,
      10, 759, 288,  16, 154, 849, 161,  10, 285,1091, 367,   0, 137,   2,  32,   4,
  };
  

  size_t length = JS::GetLinearStringLength(aKey);

  JS::AutoCheckCannotGC nogc;
  if (JS::LinearStringHasLatin1Chars(aKey)) {
    auto& entry = mozilla::perfecthash::Lookup(
      JS::GetLatin1LinearStringChars(nogc, aKey),
      length, BASES, WebIDLGlobalNameHash::sEntries);

    if (JS_LinearStringEqualsAscii(aKey, sNames + entry.mNameOffset, entry.mNameLength)) {
      return &entry;
    }
    return nullptr;
    
  } else {
    auto& entry = mozilla::perfecthash::Lookup(
      JS::GetTwoByteLinearStringChars(nogc, aKey),
      length, BASES, WebIDLGlobalNameHash::sEntries);

    if (JS_LinearStringEqualsAscii(aKey, sNames + entry.mNameOffset, entry.mNameLength)) {
      return &entry;
    }
    return nullptr;
    
  }
}

} // namespace dom
} // namespace mozilla

