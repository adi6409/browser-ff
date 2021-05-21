/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim: set ts=8 sts=2 et sw=2 tw=80: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "StaticComponents.h"

#include "mozilla/ArrayUtils.h"
#ifdef MOZ_BACKGROUNDTASKS
#  include "mozilla/BackgroundTasks.h"
#endif
#include "mozilla/PerfectHash.h"
#include "mozilla/ResultExtensions.h"
#include "mozilla/StaticPtr.h"
#include "mozilla/UniquePtr.h"
#include "mozilla/dom/ScriptSettings.h"
#include "mozJSComponentLoader.h"
#include "nsCOMPtr.h"
#include "nsComponentManager.h"
#include "nsContentUtils.h"
#include "nsIFactory.h"
#include "nsISupports.h"
#include "nsIXPConnect.h"
#include "nsString.h"
#include "nsStringEnumerator.h"
#include "nsTArray.h"
#include "xptinfo.h"

// Cleanup pollution from zipstruct.h
#undef UNSUPPORTED

// Public includes
#include "CacheStorageService.h"
#include "ContentHandlerService.h"
#include "GMPService.h"
#include "ImageBlocker.h"
#include "NSSErrorsService.h"
#include "NativeFileWatcherNotSupported.h"
#include "PaymentRequestData.h"
#include "PaymentRequestService.h"
#include "ProcessToolsService.h"
#include "RemoteAgentHandler.h"
#include "imgLoader.h"
#include "imgRequestProxy.h"
#include "mozilla/AddonContentPolicy.h"
#include "mozilla/AddonManagerStartup.h"
#include "mozilla/AlertNotification.h"
#include "mozilla/CascadeFilter.h"
#include "mozilla/ContentBlockingTelemetryService.h"
#include "mozilla/ExtensionPolicyService.h"
#include "mozilla/FOG.h"
#include "mozilla/FinalizationWitnessService.h"
#include "mozilla/LoadContextInfo.h"
#include "mozilla/MediaManager.h"
#include "mozilla/NativeOSFileInternals.h"
#include "mozilla/OSFileConstants.h"
#include "mozilla/Preferences.h"
#include "mozilla/ScriptableContentIterator.h"
#include "mozilla/StaticPtr.h"
#include "mozilla/Telemetry.h"
#include "mozilla/TextInputProcessor.h"
#include "mozilla/Viaduct.h"
#include "mozilla/appservices/AppServicesLoggerComponents.h"
#include "mozilla/dom/BlobURL.h"
#include "mozilla/dom/BlobURLProtocolHandler.h"
#include "mozilla/dom/DOMRequest.h"
#include "mozilla/dom/EventSourceEventService.h"
#include "mozilla/dom/FakeSpeechRecognitionService.h"
#include "mozilla/dom/FontTableURIProtocolHandler.h"
#include "mozilla/dom/OnlineSpeechRecognitionService.h"
#include "mozilla/dom/PresentationDeviceManager.h"
#include "mozilla/dom/PresentationTCPSessionTransport.h"
#include "mozilla/dom/PushNotifier.h"
#include "mozilla/dom/ReferrerInfo.h"
#include "mozilla/dom/SDBConnection.h"
#include "mozilla/dom/ServiceWorkerManager.h"
#include "mozilla/dom/StorageActivityService.h"
#include "mozilla/dom/WorkerDebuggerManager.h"
#include "mozilla/dom/nsCSPContext.h"
#include "mozilla/dom/nsCSPService.h"
#include "mozilla/dom/nsContentSecurityManager.h"
#include "mozilla/dom/nsMixedContentBlocker.h"
#include "mozilla/dom/nsSynthVoiceRegistry.h"
#include "mozilla/dom/quota/QuotaManagerService.h"
#include "mozilla/extensions/storage/ExtensionStorageComponents.h"
#include "mozilla/intl/LocaleService.h"
#include "mozilla/intl/OSPreferences.h"
#include "mozilla/ipc/ForkServer.h"
#include "mozilla/ipc/ForkServiceChild.h"
#include "mozilla/net/CaptivePortalService.h"
#include "mozilla/net/ChannelClassifierService.h"
#include "mozilla/net/CookieJarSettings.h"
#include "mozilla/net/Dashboard.h"
#include "mozilla/net/ExtensionProtocolHandler.h"
#include "mozilla/net/NetworkConnectivityService.h"
#include "mozilla/net/PageThumbProtocolHandler.h"
#include "mozilla/net/Predictor.h"
#include "mozilla/net/SFVService.h"
#include "mozilla/net/SubstitutingJARURI.h"
#include "mozilla/net/SubstitutingURL.h"
#include "mozilla/net/WebSocketEventService.h"
#include "mozilla/net/nsFileProtocolHandler.h"
#include "mozilla/nsMemoryInfoDumper.h"
#include "mozilla/places/History.h"
#include "mozilla/places/PageIconProtocolHandler.h"
#include "mozilla/places/SyncedBookmarksMirror.h"
#include "mozilla/widget/ScreenManager.h"
#include "nsAppRunner.h"
#include "nsApplicationCacheService.h"
#include "nsArray.h"
#include "nsBrowserStatusFilter.h"
#include "nsClientAuthRemember.h"
#include "nsConverterInputStream.h"
#include "nsDebugImpl.h"
#include "nsDeviceSensors.h"
#include "nsDirectoryService.h"
#include "nsDocLoader.h"
#include "nsExternalHelperAppService.h"
#include "nsFileStreams.h"
#include "nsGIOProtocolHandler.h"
#include "nsHTMLTags.h"
#include "nsHashPropertyBag.h"
#include "nsIPresentationService.h"
#include "nsInputStreamPump.h"
#include "nsJARURI.h"
#include "nsJSProtocolHandler.h"
#include "nsMIMEInputStream.h"
#include "nsMediaSniffer.h"
#include "nsMultiplexInputStream.h"
#include "nsNSSComponent.h"
#include "nsOSPermissionRequest.h"
#include "nsParserUtils.h"
#include "nsPluginHost.h"
#include "nsProcess.h"
#include "nsScriptSecurityManager.h"
#include "nsScriptableInputStream.h"
#include "nsSerializationHelper.h"
#include "nsSimpleNestedURI.h"
#include "nsSimpleURI.h"
#include "nsStorageStream.h"
#include "nsStructuredCloneContainer.h"
#include "nsStyleSheetService.h"
#include "nsSupportsPrimitives.h"
#include "nsSystemInfo.h"
#include "nsTerminator.h"
#include "nsURILoader.h"
#include "nsURLParsers.h"
#include "nsUrlClassifierPrefixSet.h"
#include "nsVariant.h"
#include "nsWindowWatcher.h"
#include "transport/stun_socket_filter.h"

// Relative includes
#include "../../browser/components/about/AboutRedirector.h"
#include "../../browser/components/shell/nsGNOMEShellService.h"
#include "../../caps/ContentPrincipal.h"
#include "../../caps/ExpandedPrincipal.h"
#include "../../caps/NullPrincipal.h"
#include "../../caps/SystemPrincipal.h"
#include "../../chrome/nsChromeProtocolHandler.h"
#include "../../chrome/nsChromeRegistry.h"
#include "../../devtools/platform/nsJSInspector.h"
#include "../../docshell/base/nsAboutRedirector.h"
#include "../../docshell/base/nsWebNavigationInfo.h"
#include "../../docshell/build/nsDocShellModule.h"
#include "../../dom/base/ThirdPartyUtil.h"
#include "../../dom/base/nsDataDocumentContentPolicy.h"
#include "../../dom/base/nsNoDataProtocolContentPolicy.h"
#include "../../dom/bindings/nsScriptError.h"
#include "../../dom/geolocation/Geolocation.h"
#include "../../dom/media/webspeech/synth/speechd/SpeechDispatcherService.h"
#include "../../dom/media/webspeech/synth/test/nsFakeSynthServices.h"
#include "../../dom/payments/PaymentActionResponse.h"
#include "../../dom/presentation/MockedSocketTransport.h"
#include "../../dom/presentation/provider/MulticastDNSDeviceProvider.h"
#include "../../dom/quota/nsIndexedDBProtocolHandler.h"
#include "../../dom/webbrowserpersist/nsWebBrowserPersist.h"
#include "../../extensions/permissions/PermissionDelegateHandler.h"
#include "../../extensions/permissions/PermissionManager.h"
#include "../../extensions/pref/autoconfig/src/nsReadConfig.h"
#include "../../extensions/spellcheck/src/mozPersonalDictionary.h"
#include "../../gfx/src/nsThebesFontEnumerator.h"
#include "../../image/build/nsImageModule.h"
#include "../../image/decoders/icon/nsIconProtocolHandler.h"
#include "../../image/decoders/icon/nsIconURI.h"
#include "../../image/encoders/bmp/nsBMPEncoder.h"
#include "../../image/encoders/ico/nsICOEncoder.h"
#include "../../image/encoders/jpeg/nsJPEGEncoder.h"
#include "../../image/encoders/png/nsPNGEncoder.h"
#include "../../image/imgTools.h"
#include "../../intl/locale/nsCollationFactory.h"
#include "../../intl/strres/nsStringBundleService.h"
#include "../../intl/uconv/nsConverterOutputStream.h"
#include "../../intl/uconv/nsScriptableUConv.h"
#include "../../intl/uconv/nsTextToSubURI.h"
#include "../../js/ductwork/debugger/JSDebugger.h"
#include "../../js/xpconnect/loader/mozJSSubScriptLoader.h"
#include "../../layout/build/nsLayoutModule.h"
#include "../../layout/inspector/inDeepTreeWalker.h"
#include "../../modules/libjar/nsJAR.h"
#include "../../modules/libjar/nsJARProtocolHandler.h"
#include "../../modules/libjar/zipwriter/nsDeflateConverter.h"
#include "../../modules/libjar/zipwriter/nsZipWriter.h"
#include "../../netwerk/base/ArrayBufferInputStream.h"
#include "../../netwerk/base/BackgroundFileSaver.h"
#include "../../netwerk/base/DefaultURI.h"
#include "../../netwerk/base/TLSServerSocket.h"
#include "../../netwerk/base/ThrottleQueue.h"
#include "../../netwerk/base/nsAsyncStreamCopier.h"
#include "../../netwerk/base/nsBufferedStreams.h"
#include "../../netwerk/base/nsDownloader.h"
#include "../../netwerk/base/nsIOService.h"
#include "../../netwerk/base/nsIncrementalStreamLoader.h"
#include "../../netwerk/base/nsInputStreamChannel.h"
#include "../../netwerk/base/nsNetworkInfoService.h"
#include "../../netwerk/base/nsProtocolProxyService.h"
#include "../../netwerk/base/nsServerSocket.h"
#include "../../netwerk/base/nsSimpleStreamListener.h"
#include "../../netwerk/base/nsSocketTransportService2.h"
#include "../../netwerk/base/nsStandardURL.h"
#include "../../netwerk/base/nsStreamListenerTee.h"
#include "../../netwerk/base/nsStreamLoader.h"
#include "../../netwerk/base/nsStreamTransportService.h"
#include "../../netwerk/base/nsUDPSocket.h"
#include "../../netwerk/build/nsNetModule.h"
#include "../../netwerk/cookie/CookieService.h"
#include "../../netwerk/dns/GetAddrInfo.h"
#include "../../netwerk/dns/mdns/libmdns/nsDNSServiceInfo.h"
#include "../../netwerk/dns/nsDNSService2.h"
#include "../../netwerk/dns/nsEffectiveTLDService.h"
#include "../../netwerk/dns/nsIDNService.h"
#include "../../netwerk/mime/nsMIMEHeaderParamImpl.h"
#include "../../netwerk/protocol/about/nsAboutBlank.h"
#include "../../netwerk/protocol/about/nsAboutCache.h"
#include "../../netwerk/protocol/about/nsAboutCacheEntry.h"
#include "../../netwerk/protocol/about/nsAboutProtocolHandler.h"
#include "../../netwerk/protocol/data/nsDataHandler.h"
#include "../../netwerk/protocol/ftp/nsFtpProtocolHandler.h"
#include "../../netwerk/protocol/http/nsHttpActivityDistributor.h"
#include "../../netwerk/protocol/http/nsHttpAuthManager.h"
#include "../../netwerk/protocol/res/nsResProtocolHandler.h"
#include "../../netwerk/protocol/viewsource/nsViewSourceHandler.h"
#include "../../netwerk/streamconv/converters/nsIndexedToHTML.h"
#include "../../netwerk/system/linux/nsNetworkLinkService.h"
#include "../../netwerk/wifi/nsWifiMonitor.h"
#include "../../parser/htmlparser/nsParser.h"
#include "../../security/manager/pki/nsNSSDialogs.h"
#include "../../security/manager/ssl/LocalCertService.h"
#include "../../security/manager/ssl/cert_storage/src/cert_storage.h"
#include "../../security/manager/ssl/nsCertOverrideService.h"
#include "../../security/manager/ssl/nsNSSModule.h"
#include "../../security/manager/ssl/nsNSSVersion.h"
#include "../../security/manager/ssl/nsSiteSecurityService.h"
#include "../../storage/VacuumManager.h"
#include "../../storage/mozStorageService.h"
#include "../../toolkit/components/alerts/nsAlertsService.h"
#include "../../toolkit/components/autocomplete/nsAutoCompleteController.h"
#include "../../toolkit/components/autocomplete/nsAutoCompleteSimpleResult.h"
#include "../../toolkit/components/browser/nsWebBrowserContentPolicy.h"
#include "../../toolkit/components/build/nsToolkitCompsModule.h"
#include "../../toolkit/components/ctypes/ctypes.h"
#include "../../toolkit/components/downloads/DownloadPlatform.h"
#include "../../toolkit/components/find/nsFind.h"
#include "../../toolkit/components/find/nsFindService.h"
#include "../../toolkit/components/mozintl/MozIntlHelper.h"
#include "../../toolkit/components/parentalcontrols/nsParentalControlsService.h"
#include "../../toolkit/components/places/nsAnnoProtocolHandler.h"
#include "../../toolkit/components/places/nsFaviconService.h"
#include "../../toolkit/components/places/nsNavBookmarks.h"
#include "../../toolkit/components/places/nsNavHistory.h"
#include "../../toolkit/components/reflect/reflect.h"
#include "../../toolkit/components/reputationservice/ApplicationReputation.h"
#include "../../toolkit/components/reputationservice/LoginReputation.h"
#include "../../toolkit/components/satchel/nsFormFillController.h"
#include "../../toolkit/components/startup/nsAppStartup.h"
#include "../../toolkit/components/typeaheadfind/nsTypeAheadFind.h"
#include "../../toolkit/components/url-classifier/nsUrlClassifierStreamUpdater.h"
#include "../../toolkit/components/url-classifier/nsUrlClassifierUtils.h"
#include "../../toolkit/components/windowwatcher/nsDialogParamBlock.h"
#include "../../toolkit/system/gnome/nsGIOService.h"
#include "../../toolkit/system/gnome/nsGSettingsService.h"
#include "../../toolkit/system/gnome/nsSystemAlertsService.h"
#include "../../toolkit/xre/nsUpdateDriver.h"
#include "../../toolkit/xre/nsUpdateSyncManager.h"
#include "../../toolkit/xre/nsXREDirProvider.h"
#include "../../tools/profiler/gecko/nsProfiler.h"
#include "../../uriloader/exthandler/nsDBusHandlerApp.h"
#include "../../uriloader/exthandler/nsExternalProtocolHandler.h"
#include "../../uriloader/exthandler/nsLocalHandlerApp.h"
#include "../../uriloader/prefetch/nsOfflineCacheUpdate.h"
#include "../../uriloader/prefetch/nsPrefetchService.h"
#include "../../widget/GfxInfoX11.h"
#include "../../widget/gtk/TaskbarProgress.h"
#include "../../widget/gtk/nsApplicationChooser.h"
#include "../../widget/gtk/nsColorPicker.h"
#include "../../widget/gtk/nsDeviceContextSpecG.h"
#include "../../widget/gtk/nsDragService.h"
#include "../../widget/gtk/nsFilePicker.h"
#include "../../widget/gtk/nsImageToPixbuf.h"
#include "../../widget/gtk/nsPrintDialogGTK.h"
#include "../../widget/gtk/nsPrintSettingsServiceGTK.h"
#include "../../widget/gtk/nsSound.h"
#include "../../widget/gtk/nsUserIdleServiceGTK.h"
#include "../../widget/gtk/nsWidgetFactory.h"
#include "../../widget/nsClipboardHelper.h"
#include "../../widget/nsHTMLFormatConverter.h"
#include "../../widget/nsPrintSession.h"
#include "../../widget/nsPrinterListCUPS.h"
#include "../../widget/nsTransferable.h"
#include "../../xpcom/base/nsConsoleService.h"
#include "../../xpcom/base/nsMemoryImpl.h"
#include "../../xpcom/base/nsMemoryReporterManager.h"
#include "../../xpcom/base/nsMessageLoop.h"
#include "../../xpcom/base/nsSecurityConsoleMessage.h"
#include "../../xpcom/base/nsUUIDGenerator.h"
#include "../../xpcom/base/nsVersionComparatorImpl.h"
#include "../../xpcom/build/XPCOMModule.h"
#include "../../xpcom/components/nsCategoryManager.h"
#include "../../xpcom/components/nsComponentManager.h"
#include "../../xpcom/ds/nsObserverService.h"
#include "../../xpcom/io/nsBinaryStream.h"
#include "../../xpcom/io/nsIOUtil.h"
#include "../../xpcom/io/nsPipe.h"
#include "../../xpcom/io/nsScriptableBase64Encoder.h"
#include "../../xpcom/threads/nsEnvironment.h"
#include "../../xpcom/threads/nsTimerImpl.h"
#include "../../xpfe/appshell/nsAppShellService.h"
#include "../../xpfe/appshell/nsWindowMediator.h"

class ContentSignatureVerifier;
class OSKeyStore;
class OSReauthenticator;
class SecretDecoderRing;
class mozHunspell;
class mozISandboxReporter;
class mozISandboxSettings;
class nsCertTree;
class nsCryptoHMAC;
class nsCryptoHash;
class nsIAccessibilityService;
class nsIClipboard;
class nsIContentSerializer;
class nsIContentViewer;
class nsIDocumentLoaderFactory;
class nsIEventListenerService;
class nsIFocusManager;
class nsIFrameTraversal;
class nsIIdentityCryptoService;
class nsIStartupCacheInfo;
class nsISupports;
class nsITelemetry;
class nsKeyObject;
class nsKeyObjectFactory;
class nsNSSCertificate;
class nsNSSCertificateDB;
class nsPK11TokenDB;
class nsPrefLocalizedString;
class nsRandomGenerator;
class nsUnixSystemProxySettings;
namespace mozilla {
class PeerConnectionImpl;
namespace net {
class nsHttpHandler;
class nsHttpsHandler;
}  // namespace net
namespace psm {
class PKCS11ModuleDB;
class TransportSecurityInfo;
}  // namespace psm
}  // namespace mozilla

namespace mozilla {

using dom::AutoJSAPI;

namespace xpcom {

static constexpr uint32_t kNoContractID = 0xffffffff;

namespace {
// Template helpers for constructor function sanity checks.
template <typename T>
struct RemoveAlreadyAddRefed {
  using Type = T;
};

template <typename T>
struct RemoveAlreadyAddRefed<already_AddRefed<T>> {
  using Type = T;
};
}  // anonymous namespace


uint8_t gInvalidContracts[kContractCount / 8 + 1];

static StaticRefPtr<nsISupports> gServiceInstances[kStaticModuleCount];

uint8_t gInitCalled[kModuleInitCount / 8 + 1];

static const char gStrings[] =
    /* 0x0 */ "Gecko-Content-Viewers\0"
    /* 0x16 */ "application/ecmascript\0"
    /* 0x2d */ "@mozilla.org/content/document-loader-factory;1\0"
    /* 0x5c */ "application/http-index-format\0"
    /* 0x7a */ "@mozilla.org/content-viewers/http-index-format\0"
    /* 0xa9 */ "application/javascript\0"
    /* 0xc0 */ "application/json\0"
    /* 0xd1 */ "application/mathml+xml\0"
    /* 0xe8 */ "application/rdf+xml\0"
    /* 0xfc */ "application/vnd.mozilla.xul+xml\0"
    /* 0x11c */ "application/vnd.wap.xhtml+xml\0"
    /* 0x13a */ "application/x-javascript\0"
    /* 0x153 */ "application/x-view-source\0"
    /* 0x16d */ "application/xhtml+xml\0"
    /* 0x183 */ "application/xml\0"
    /* 0x193 */ "image/apng\0"
    /* 0x19e */ "image/bmp\0"
    /* 0x1a8 */ "image/gif\0"
    /* 0x1b2 */ "image/icon\0"
    /* 0x1bd */ "image/jpeg\0"
    /* 0x1c8 */ "image/jpg\0"
    /* 0x1d2 */ "image/pjpeg\0"
    /* 0x1de */ "image/png\0"
    /* 0x1e8 */ "image/svg+xml\0"
    /* 0x1f6 */ "image/vnd.microsoft.icon\0"
    /* 0x20f */ "image/x-icon\0"
    /* 0x21c */ "image/x-ms-bmp\0"
    /* 0x22b */ "image/x-png\0"
    /* 0x237 */ "text/cache-manifest\0"
    /* 0x24b */ "text/css\0"
    /* 0x254 */ "text/ecmascript\0"
    /* 0x264 */ "text/html\0"
    /* 0x26e */ "text/javascript\0"
    /* 0x27e */ "text/json\0"
    /* 0x288 */ "text/plain\0"
    /* 0x293 */ "text/rdf\0"
    /* 0x29c */ "text/vtt\0"
    /* 0x2a5 */ "text/xml\0"
    /* 0x2ae */ "app-startup\0"
    /* 0x2ba */ "FormFillController\0"
    /* 0x2cd */ "@mozilla.org/satchel/form-fill-controller;1\0"
    /* 0x2f9 */ "TelemetryControllerContent\0"
    /* 0x314 */ "@mozilla.org/base/telemetry-controller-content;1\0"
    /* 0x345 */ "command-line-handler\0"
    /* 0x35a */ "m-browser\0"
    /* 0x364 */ "@mozilla.org/browser/clh;1\0"
    /* 0x37f */ "m-devtools\0"
    /* 0x38a */ "@mozilla.org/devtools/startup-clh;1\0"
    /* 0x3ae */ "m-recording\0"
    /* 0x3ba */ "@mozilla.org/commandlinehandler/general-startup;1?type=recording\0"
    /* 0x3fb */ "m-remote\0"
    /* 0x404 */ "@mozilla.org/commandlinehandler/general-startup;1?type=remote\0"
    /* 0x442 */ "x-default\0"
    /* 0x44c */ "@mozilla.org/browser/final-clh;1\0"
    /* 0x46d */ "y-default\0"
    /* 0x477 */ "@mozilla.org/toolkit/default-clh;1\0"
    /* 0x49a */ "command-line-validator\0"
    /* 0x4b1 */ "b-browser\0"
    /* 0x4bb */ "content-policy\0"
    /* 0x4ca */ "@mozilla.org/addons/content-policy;1\0"
    /* 0x4ef */ "@mozilla.org/data-document-content-policy;1\0"
    /* 0x51b */ "@mozilla.org/embedding/browser/content-policy;1\0"
    /* 0x54b */ "@mozilla.org/image-blocker-content-policy;1\0"
    /* 0x577 */ "@mozilla.org/mixedcontentblocker;1\0"
    /* 0x59a */ "@mozilla.org/no-data-protocol-content-policy;1\0"
    /* 0x5c9 */ "CSPService\0"
    /* 0x5d4 */ "@mozilla.org/cspservice;1\0"
    /* 0x5ee */ "content-sniffing-services\0"
    /* 0x608 */ "@mozilla.org/image/loader;1\0"
    /* 0x624 */ "@mozilla.org/image/cache;1\0"
    /* 0x63f */ "@mozilla.org/media/sniffer;1\0"
    /* 0x65c */ "idle-daily\0"
    /* 0x667 */ "ContentBlockingTelemetryService\0"
    /* 0x687 */ "@mozilla.org/content-blocking-telemetry-service;1\0"
    /* 0x6b9 */ "MozStorage Vacuum Manager\0"
    /* 0x6d3 */ "@mozilla.org/storage/vacuum;1\0"
    /* 0x6f1 */ "PlacesDBUtilsIdleMaintenance\0"
    /* 0x70e */ "@mozilla.org/places/databaseUtilsIdleMaintenance;1\0"
    /* 0x741 */ "QuotaManagerService\0"
    /* 0x755 */ "@mozilla.org/dom/quota-manager-service;1\0"
    /* 0x77e */ "net-channel-event-sinks\0"
    /* 0x796 */ "net-content-sniffers\0"
    /* 0x7ab */ "JSONView\0"
    /* 0x7b4 */ "@mozilla.org/devtools/jsonview-sniffer;1\0"
    /* 0x7dd */ "payment-request\0"
    /* 0x7ed */ "BasicCardMethodChangeDetails\0"
    /* 0x80a */ "@mozilla.org/dom/payments/basiccard-change-details;1\0"
    /* 0x83f */ "BasicCardResponseData\0"
    /* 0x855 */ "@mozilla.org/dom/payments/basiccard-response-data;1\0"
    /* 0x889 */ "GeneralMethodChangeDetails\0"
    /* 0x8a4 */ "@mozilla.org/dom/payments/general-change-details;1\0"
    /* 0x8d7 */ "GeneralResponseData\0"
    /* 0x8eb */ "@mozilla.org/dom/payments/general-response-data;1\0"
    /* 0x91d */ "PaymentAbortActionResponse\0"
    /* 0x938 */ "@mozilla.org/dom/payments/payment-abort-action-response;1\0"
    /* 0x972 */ "PaymentAddress\0"
    /* 0x981 */ "@mozilla.org/dom/payments/payment-address;1\0"
    /* 0x9ad */ "PaymentCanMakeActionResponse\0"
    /* 0x9ca */ "@mozilla.org/dom/payments/payment-canmake-action-response;1\0"
    /* 0xa06 */ "PaymentCompleteActionResponse\0"
    /* 0xa24 */ "@mozilla.org/dom/payments/payment-complete-action-response;1\0"
    /* 0xa61 */ "PaymentRequestService\0"
    /* 0xa77 */ "@mozilla.org/dom/payments/payment-request-service;1\0"
    /* 0xaab */ "PaymentShowActionResponse\0"
    /* 0xac5 */ "@mozilla.org/dom/payments/payment-show-action-response;1\0"
    /* 0xafe */ "places-init-complete\0"
    /* 0xb13 */ "nsPlacesExpiration\0"
    /* 0xb26 */ "@mozilla.org/places/expiration;1\0"
    /* 0xb47 */ "pref-config-startup\0"
    /* 0xb5b */ "ReadConfig Module\0"
    /* 0xb6d */ "@mozilla.org/readconfig;1\0"
    /* 0xb87 */ "profile-after-change\0"
    /* 0xb9c */ "CrashMonitor\0"
    /* 0xba9 */ "@mozilla.org/toolkit/crashmonitor;1\0"
    /* 0xbcd */ "CrashService\0"
    /* 0xbda */ "@mozilla.org/crashservice;1\0"
    /* 0xbf6 */ "PresentationDeviceManager\0"
    /* 0xc10 */ "@mozilla.org/presentation-device/manager;1\0"
    /* 0xc3b */ "PresentationService\0"
    /* 0xc4f */ "@mozilla.org/presentation/presentationservice;1\0"
    /* 0xc7f */ "PurgeTrackerService\0"
    /* 0xc93 */ "@mozilla.org/purge-tracker-service;1\0"
    /* 0xcb8 */ "formHistoryStartup\0"
    /* 0xccb */ "@mozilla.org/satchel/form-history-startup;1\0"
    /* 0xcf7 */ "nsTerminatorTelemetry\0"
    /* 0xd0d */ "@mozilla.org/toolkit/shutdown-terminator-telemetry;1\0"
    /* 0xd42 */ "nsUpdateServiceStub\0"
    /* 0xd56 */ "@mozilla.org/updates/update-service-stub;1\0"
    /* 0xd81 */ "nsUpdateTimerManager\0"
    /* 0xd96 */ "@mozilla.org/updates/timer-manager;1\0"
    /* 0xdbb */ "push\0"
    /* 0xdc0 */ "chrome://fxa-device-update\0"
    /* 0xddb */ "@mozilla.org/fxaccounts/push;1\0"
    /* 0xdfa */ "speech-synth-started\0"
    /* 0xe0f */ "Fake Speech Synth\0"
    /* 0xe21 */ "@mozilla.org/fakesynth;1\0"
    /* 0xe3a */ "SpeechDispatcher Speech Synth\0"
    /* 0xe58 */ "@mozilla.org/synthspeechdispatcher;1\0"
    /* 0xe7d */ "vacuum-participant\0"
    /* 0xe90 */ "Places\0"
    /* 0xe97 */ "@mozilla.org/browser/nav-history-service;1\0"
    /* 0xec2 */ "xpcom-startup\0"
    /* 0xed0 */ "Fork Server Launcher\0"
    /* 0xee5 */ "@mozilla.org/fork-server-launcher;1\0"
    /* 0xf09 */ "nsUpdateSyncManager\0"
    /* 0xf1d */ "@mozilla.org/updates/update-sync-manager;1\0"
    /* 0xf48 */ "chrome://remote/content/components/RemoteAgent.jsm\0"
    /* 0xf7b */ "resource:///modules/AboutDebuggingRegistration.jsm\0"
    /* 0xfae */ "resource:///modules/AboutDevToolsRegistration.jsm\0"
    /* 0xfe0 */ "resource:///modules/AboutDevToolsToolboxRegistration.jsm\0"
    /* 0x1019 */ "resource:///modules/AboutNewTabService.jsm\0"
    /* 0x1044 */ "resource:///modules/BrowserContentHandler.jsm\0"
    /* 0x1072 */ "resource:///modules/BrowserGlue.jsm\0"
    /* 0x1096 */ "resource:///modules/ChromeProfileMigrator.jsm\0"
    /* 0x10c4 */ "resource:///modules/DevToolsStartup.jsm\0"
    /* 0x10ec */ "resource:///modules/FirefoxProfileMigrator.jsm\0"
    /* 0x111b */ "resource:///modules/ProfileMigrator.jsm\0"
    /* 0x1143 */ "resource:///modules/PromptCollection.jsm\0"
    /* 0x116c */ "resource:///modules/WebProtocolHandlerRegistrar.jsm\0"
    /* 0x11a0 */ "resource://devtools/client/jsonview/Converter.jsm\0"
    /* 0x11d2 */ "resource://devtools/client/jsonview/Sniffer.jsm\0"
    /* 0x1202 */ "resource://gre/modules/AutoCompleteSimpleSearch.jsm\0"
    /* 0x1236 */ "resource://gre/modules/BrowserElementParent.jsm\0"
    /* 0x1266 */ "resource://gre/modules/CaptiveDetect.jsm\0"
    /* 0x128f */ "resource://gre/modules/ClearDataService.jsm\0"
    /* 0x12bb */ "resource://gre/modules/ConsoleAPIStorage.jsm\0"
    /* 0x12e8 */ "resource://gre/modules/ContentAreaDropListener.jsm\0"
    /* 0x131b */ "resource://gre/modules/ContentDispatchChooser.jsm\0"
    /* 0x134d */ "resource://gre/modules/ContentPrefService2.jsm\0"
    /* 0x137c */ "resource://gre/modules/CrashManager.jsm\0"
    /* 0x13a4 */ "resource://gre/modules/CrashService.jsm\0"
    /* 0x13cc */ "resource://gre/modules/DNSServiceDiscovery.jsm\0"
    /* 0x13fb */ "resource://gre/modules/DefaultCLH.jsm\0"
    /* 0x1421 */ "resource://gre/modules/DotProtocolHandler.jsm\0"
    /* 0x144f */ "resource://gre/modules/DownloadLegacy.jsm\0"
    /* 0x1479 */ "resource://gre/modules/EnterprisePolicies.jsm\0"
    /* 0x14a7 */ "resource://gre/modules/ExtensionStorageComponents.jsm\0"
    /* 0x14dd */ "resource://gre/modules/FormAutoComplete.jsm\0"
    /* 0x1509 */ "resource://gre/modules/FormHistoryStartup.jsm\0"
    /* 0x1537 */ "resource://gre/modules/FxAccountsPush.jsm\0"
    /* 0x1561 */ "resource://gre/modules/HTMLMenuBuilder.jsm\0"
    /* 0x158c */ "resource://gre/modules/HelperAppDlg.jsm\0"
    /* 0x15b4 */ "resource://gre/modules/InputListAutoComplete.jsm\0"
    /* 0x15e5 */ "resource://gre/modules/LoginAutoComplete.jsm\0"
    /* 0x1612 */ "resource://gre/modules/LoginInfo.jsm\0"
    /* 0x1637 */ "resource://gre/modules/LoginManager.jsm\0"
    /* 0x165f */ "resource://gre/modules/LoginManagerAuthPrompter.jsm\0"
    /* 0x1693 */ "resource://gre/modules/LoginManagerPrompter.jsm\0"
    /* 0x16c3 */ "resource://gre/modules/MainProcessSingleton.jsm\0"
    /* 0x16f3 */ "resource://gre/modules/MozProtocolHandler.jsm\0"
    /* 0x1721 */ "resource://gre/modules/NetworkGeolocationProvider.jsm\0"
    /* 0x1757 */ "resource://gre/modules/NotificationStorage.jsm\0"
    /* 0x1786 */ "resource://gre/modules/PageThumbsStorageService.jsm\0"
    /* 0x17ba */ "resource://gre/modules/PartitioningExceptionListService.jsm\0"
    /* 0x17f6 */ "resource://gre/modules/PlacesDBUtils.jsm\0"
    /* 0x181f */ "resource://gre/modules/PlacesExpiration.jsm\0"
    /* 0x184b */ "resource://gre/modules/PresentationControlService.jsm\0"
    /* 0x1881 */ "resource://gre/modules/PresentationDataChannelSessionTransport.jsm\0"
    /* 0x18c4 */ "resource://gre/modules/ProcessSelector.jsm\0"
    /* 0x18ef */ "resource://gre/modules/Prompter.jsm\0"
    /* 0x1913 */ "resource://gre/modules/PurgeTrackerService.jsm\0"
    /* 0x1942 */ "resource://gre/modules/Push.jsm\0"
    /* 0x1962 */ "resource://gre/modules/PushComponents.jsm\0"
    /* 0x198c */ "resource://gre/modules/RecordingCmdLine.jsm\0"
    /* 0x19b8 */ "resource://gre/modules/SearchService.jsm\0"
    /* 0x19e1 */ "resource://gre/modules/SearchSuggestions.jsm\0"
    /* 0x1a0e */ "resource://gre/modules/ShieldContentProcess.jsm\0"
    /* 0x1a3e */ "resource://gre/modules/Sidebar.jsm\0"
    /* 0x1a61 */ "resource://gre/modules/SimpleServices.jsm\0"
    /* 0x1a8b */ "resource://gre/modules/SlowScriptDebug.jsm\0"
    /* 0x1ab6 */ "resource://gre/modules/TaggingService.jsm\0"
    /* 0x1ae0 */ "resource://gre/modules/TelemetryControllerContent.jsm\0"
    /* 0x1b16 */ "resource://gre/modules/TelemetryStartup.jsm\0"
    /* 0x1b42 */ "resource://gre/modules/TerminatorTelemetry.jsm\0"
    /* 0x1b71 */ "resource://gre/modules/TooltipTextProvider.jsm\0"
    /* 0x1ba0 */ "resource://gre/modules/TrackingDBService.jsm\0"
    /* 0x1bcd */ "resource://gre/modules/URIFixup.jsm\0"
    /* 0x1bf1 */ "resource://gre/modules/URLDecorationAnnotationsService.jsm\0"
    /* 0x1c2c */ "resource://gre/modules/URLFormatter.jsm\0"
    /* 0x1c54 */ "resource://gre/modules/UnifiedComplete.jsm\0"
    /* 0x1c7f */ "resource://gre/modules/UpdateService.jsm\0"
    /* 0x1ca8 */ "resource://gre/modules/UpdateServiceStub.jsm\0"
    /* 0x1cd5 */ "resource://gre/modules/UpdateTimerManager.jsm\0"
    /* 0x1d03 */ "resource://gre/modules/UrlClassifierExceptionListService.jsm\0"
    /* 0x1d40 */ "resource://gre/modules/UrlClassifierHashCompleter.jsm\0"
    /* 0x1d76 */ "resource://gre/modules/UrlClassifierLib.jsm\0"
    /* 0x1da2 */ "resource://gre/modules/UrlClassifierListManager.jsm\0"
    /* 0x1dd6 */ "resource://gre/modules/WebHandlerApp.jsm\0"
    /* 0x1dff */ "resource://gre/modules/WebVTTParserWrapper.jsm\0"
    /* 0x1e2e */ "resource://gre/modules/WellKnownOpportunisticUtils.jsm\0"
    /* 0x1e65 */ "resource://gre/modules/XULStore.jsm\0"
    /* 0x1e89 */ "resource://gre/modules/addonManager.js\0"
    /* 0x1eb0 */ "resource://gre/modules/amContentHandler.jsm\0"
    /* 0x1edc */ "resource://gre/modules/amInstallTrigger.jsm\0"
    /* 0x1f08 */ "resource://gre/modules/amWebAPI.jsm\0"
    /* 0x1f2c */ "resource://gre/modules/crypto-SDR.js\0"
    /* 0x1f51 */ "resource://gre/modules/media/PeerConnection.jsm\0"
    /* 0x1f81 */ "resource://gre/modules/mozIntl.jsm\0"
    /* 0x1fa4 */ "resource://gre/modules/nsAsyncShutdown.jsm\0"
    /* 0x1fcf */ "resource://gre/modules/nsCrashMonitor.jsm\0"
    /* 0x1ff9 */ "resource://gre/modules/pdfjs.js\0"
    /* 0x2019 */ "resource://gre/modules/storage-json.js\0"
    /* 0x2040 */ "resource://services-settings/RemoteSettingsComponents.jsm\0"
    /* 0x207a */ "resource://services-sync/Weave.jsm\0"
    /* 0x209d */ "resource://webcompat/AboutCompat.jsm\0"
    /* 0x20c2 */ "@mozilla.org/widget/clipboard;1\0"
    /* 0x20e2 */ "@mozilla.org/system-info;1\0"
    /* 0x20fd */ "@mozilla.org/embedcomp/window-watcher;1\0"
    /* 0x2125 */ "@mozilla.org/alerts-service;1\0"
    /* 0x2143 */ "@mozilla.org/xre/app-info;1\0"
    /* 0x215f */ "@mozilla.org/content-permission/prompt;1\0"
    /* 0x2188 */ "@mozilla.org/appservices/logger;1\0"
    /* 0x21aa */ "@mozilla.org/reputationservice/application-reputation-service;1\0"
    /* 0x21ea */ "@mozilla.org/uriloader/content-handler;1?type=image/x-icon\0"
    /* 0x2225 */ "@mozilla.org/profile/migrator;1?app=browser&type=chrome-dev\0"
    /* 0x2261 */ "@mozilla.org/streamconv;1?from=application/octet-stream&to=*/*\0"
    /* 0x22a0 */ "@mozilla.org/security/random-generator;1\0"
    /* 0x22c9 */ "@mozilla.org/appshell/appShellService;1\0"
    /* 0x22f1 */ "@mozilla.org/updates/update-checker;1\0"
    /* 0x2317 */ "@mozilla.org/autocomplete/controller;1\0"
    /* 0x233e */ "@mozilla.org/security/hash;1\0"
    /* 0x235b */ "@mozilla.org/embedcomp/rangefind;1\0"
    /* 0x237e */ "@mozilla.org/layout/contentserializer;1?mimetype=text/xml\0"
    /* 0x23b8 */ "@mozilla.org/network/protocol/about;1?what=history\0"
    /* 0x23eb */ "@mozilla.org/browser/favicon-service;1\0"
    /* 0x2412 */ "@mozilla.org/memory-info-dumper;1\0"
    /* 0x2434 */ "@mozilla.org/network/async-stream-copier;1\0"
    /* 0x245f */ "@mozilla.org/login-manager/crypto/SDR;1\0"
    /* 0x2487 */ "@mozilla.org/presentation/control-service;1\0"
    /* 0x24b3 */ "@mozilla.org/supports-PRUint32;1\0"
    /* 0x24d4 */ "@mozilla.org/network/mime-hdrparam;1\0"
    /* 0x24f9 */ "@mozilla.org/network/url-parser;1?auth=maybe\0"
    /* 0x2526 */ "@mozilla.org/toolkit/osfile/native-internals;1\0"
    /* 0x2555 */ "@mozilla.org/addon-web-api/manager;1\0"
    /* 0x257a */ "@mozilla.org/network/url-parser;1?auth=no\0"
    /* 0x25a4 */ "@mozilla.org/identity/crypto-service;1\0"
    /* 0x25cb */ "@mozilla.org/network/buffered-output-stream;1\0"
    /* 0x25f9 */ "@mozilla.org/intl/localeservice;1\0"
    /* 0x261b */ "@mozilla.org/layout/contentserializer;1?mimetype=text/html\0"
    /* 0x2656 */ "@mozilla.org/dom/rtcdtmfsender;1\0"
    /* 0x2677 */ "@mozilla.org/network/idn-service;1\0"
    /* 0x269a */ "@mozilla.org/io/arraybuffer-input-stream;1\0"
    /* 0x26c5 */ "@mozilla.org/security/oskeystore;1\0"
    /* 0x26e8 */ "@mozilla.org/gfx/printsettings-service;1\0"
    /* 0x2711 */ "@mozilla.org/thirdpartyutil;1\0"
    /* 0x272f */ "@mozilla.org/extensions/storage/internal/sync-area;1\0"
    /* 0x2764 */ "@mozilla.org/autocomplete/search;1?name=search-autocomplete\0"
    /* 0x27a0 */ "@mozilla.org/geolocation/provider;1\0"
    /* 0x27c4 */ "@mozilla.org/widget/image-to-gdk-pixbuf;1\0"
    /* 0x27ee */ "@mozilla.org/docloaderservice;1\0"
    /* 0x280e */ "@mozilla.org/network/protocol/about;1?what=certerror\0"
    /* 0x2843 */ "@mozilla.org/gfx/info;1\0"
    /* 0x285b */ "@mozilla.org/network/util;1\0"
    /* 0x2877 */ "@mozilla.org/url-classifier/listmanager;1\0"
    /* 0x28a1 */ "@mozilla.org/widget/transferable;1\0"
    /* 0x28c4 */ "@mozilla.org/browser/tagging-service;1\0"
    /* 0x28eb */ "@mozilla.org/network/protocol/about;1?what=loginsimportreport\0"
    /* 0x2929 */ "@mozilla.org/supports-float;1\0"
    /* 0x2947 */ "@mozilla.org/dom/workers/workerdebuggermanager;1\0"
    /* 0x2978 */ "@mozilla.org/updates/update-manager;1\0"
    /* 0x299e */ "@mozilla.org/memory-reporter-manager;1\0"
    /* 0x29c5 */ "@mozilla.org/network/udp-filter-handler;1?name=stun\0"
    /* 0x29f9 */ "@mozilla.org/observer-service;1\0"
    /* 0x2a19 */ "@mozilla.org/serviceworkers/manager;1\0"
    /* 0x2a3f */ "@mozilla.org/alert-notification;1\0"
    /* 0x2a61 */ "@mozilla.org/autocomplete/simple-result;1\0"
    /* 0x2a8b */ "@mozilla.org/url-classifier/jslib;1\0"
    /* 0x2aaf */ "@mozilla.org/network/network-link-service;1\0"
    /* 0x2adb */ "@mozilla.org/jsdebugger;1\0"
    /* 0x2af5 */ "@mozilla.org/network/protocol;1?name=default\0"
    /* 0x2b22 */ "@mozilla.org/docshell/structured-clone-container;1\0"
    /* 0x2b55 */ "@mozilla.org/streamconv;1?from=uncompressed&to=x-gzip\0"
    /* 0x2b8b */ "@mozilla.org/chrome/chrome-registry;1\0"
    /* 0x2bb1 */ "@mozilla.org/network/protocol/about;1?what=glean\0"
    /* 0x2be2 */ "@mozilla.org/network/protocol;1?name=moz-page-thumb\0"
    /* 0x2c16 */ "@mozilla.org/network/atomic-file-output-stream;1\0"
    /* 0x2c47 */ "@mozilla.org/remote/agent;1\0"
    /* 0x2c63 */ "@mozilla.org/network/serialization-helper;1\0"
    /* 0x2c8f */ "@mozilla.org/url-classifier/exception-list-service;1\0"
    /* 0x2cc4 */ "@mozilla.org/network/protocol/about;1?what=protections\0"
    /* 0x2cfb */ "@mozilla.org/http-sfv-service;1\0"
    /* 0x2d1b */ "@mozilla.org/url-classifier/utils;1\0"
    /* 0x2d3f */ "@mozilla.org/processtools-service;1\0"
    /* 0x2d63 */ "@mozilla.org/intl/stringbundle;1\0"
    /* 0x2d84 */ "@mozilla.org/streamconv;1?from=text/ftp-dir&to=application/http-index-format\0"
    /* 0x2dd1 */ "@mozilla.org/dom/peerconnectionobserver;1\0"
    /* 0x2dfb */ "@mozilla.org/gfx/screenmanager;1\0"
    /* 0x2e1c */ "@mozilla.org/scriptablebase64encoder;1\0"
    /* 0x2e43 */ "@mozilla.org/find/find_service;1\0"
    /* 0x2e64 */ "@mozilla.org/toolkit/components/mdnsresponder/dns-sd;1\0"
    /* 0x2e9b */ "@mozilla.org/system-alerts-service;1\0"
    /* 0x2ec0 */ "@mozilla.org/xul/xulstore;1\0"
    /* 0x2edc */ "@mozilla.org/streamconv;1?from=uncompressed&to=deflate\0"
    /* 0x2f13 */ "@mozilla.org/netwerk/cache-storage-service;1\0"
    /* 0x2f40 */ "@mozilla.org/partitioning/exception-list-service;1\0"
    /* 0x2f73 */ "@mozilla.org/typeaheadfind;1\0"
    /* 0x2f90 */ "@mozilla.org/streamconv;1?from=application/vnd.mozilla.json.view&to=*/*\0"
    /* 0x2fd8 */ "@mozilla.org/embedcomp/dialogparam;1\0"
    /* 0x2ffd */ "@mozilla.org/streamconv;1?from=gzip&to=uncompressed\0"
    /* 0x3031 */ "@mozilla.org/streamconv;1?from=br&to=uncompressed\0"
    /* 0x3063 */ "@mozilla.org/network/protocol/about;1?what=certificate\0"
    /* 0x309a */ "@mozilla.org/streamconv;1?from=application/x-unknown-content-type&to=*/*\0"
    /* 0x30e3 */ "@mozilla.org/toolkit/finalizationwitness;1\0"
    /* 0x310e */ "@mozilla.org/network/protocol/about;1?what=pocket-saved\0"
    /* 0x3146 */ "@mozilla.org/network/protocol/about;1?what=memory\0"
    /* 0x3178 */ "@mozilla.org/geolocation;1\0"
    /* 0x3193 */ "@mozilla.org/embedcomp/default-tooltiptextprovider;1\0"
    /* 0x31c8 */ "@mozilla.org/intl/converter-output-stream;1\0"
    /* 0x31f4 */ "@mozilla.org/dom/rtcsessiondescription;1\0"
    /* 0x321d */ "@mozilla.org/uriloader/handler-service;1\0"
    /* 0x3246 */ "@mozilla.org/network/binary-detector;1\0"
    /* 0x326d */ "@mozilla.org/gfx/printsession;1\0"
    /* 0x328d */ "@mozilla.org/network/protocol;1?name=blob\0"
    /* 0x32b7 */ "@mozilla.org/network/network-connectivity-service;1\0"
    /* 0x32eb */ "@mozilla.org/network/server-socket;1\0"
    /* 0x3310 */ "@mozilla.org/referrer-info;1\0"
    /* 0x332d */ "@mozilla.org/supports-double;1\0"
    /* 0x334c */ "@mozilla.org/toolkit/shell-service;1\0"
    /* 0x3371 */ "@mozilla.org/network/protocol/about;1?what=checkerboard\0"
    /* 0x33a9 */ "@mozilla.org/network/throttlequeue;1\0"
    /* 0x33ce */ "@mozilla.org/intl/ospreferences;1\0"
    /* 0x33f0 */ "@mozilla.org/network/protocol;1?name=http\0"
    /* 0x341a */ "@mozilla.org/process/environment;1\0"
    /* 0x343d */ "@mozilla.org/network/protocol;1?name=javascript\0"
    /* 0x346d */ "@mozilla.org/toolkit/app-startup;1\0"
    /* 0x3490 */ "@mozilla.org/eventlistenerservice;1\0"
    /* 0x34b4 */ "@mozilla.org/network/stream-loader;1\0"
    /* 0x34d9 */ "@mozilla.org/autocomplete/search;1?name=login-doorhanger-username\0"
    /* 0x351b */ "@mozilla.org/network/socket-transport-service;1\0"
    /* 0x354b */ "@mozilla.org/network/effective-tld-service;1\0"
    /* 0x3578 */ "@mozilla.org/ssservice;1\0"
    /* 0x3591 */ "@mozilla.org/dom/localStorage-manager;1\0"
    /* 0x35b9 */ "@mozilla.org/network/udp-socket;1\0"
    /* 0x35db */ "@mozilla.org/dom/rtpreceiver;1\0"
    /* 0x35fa */ "@mozilla.org/network-info-service;1\0"
    /* 0x361e */ "@mozilla.org/xre/directory-provider;1\0"
    /* 0x3644 */ "@mozilla.org/binaryoutputstream;1\0"
    /* 0x3666 */ "@mozilla.org/layout/contentserializer;1?mimetype=application/xhtml+xml\0"
    /* 0x36ad */ "@mozilla.org/uriloader/content-handler;1?type=image/svg+xml\0"
    /* 0x36e9 */ "@mozilla.org/profile/migrator;1?app=browser&type=firefox\0"
    /* 0x3722 */ "@mozilla.org/dom/peerconnectionstatic;1\0"
    /* 0x374a */ "@mozilla.org/dom/dom-request-service;1\0"
    /* 0x3771 */ "@mozilla.org/supports-PRUint16;1\0"
    /* 0x3792 */ "@mozilla.org/crashmanager;1\0"
    /* 0x37ae */ "@mozilla.org/streamconv;1?from=compress&to=uncompressed\0"
    /* 0x37e6 */ "@mozilla.org/network/well-known-opportunistic-utils;1\0"
    /* 0x381c */ "@mozilla.org/gsettings-service;1\0"
    /* 0x383d */ "@mozilla.org/security/certoverride;1\0"
    /* 0x3862 */ "@mozilla.org/toolkit/captive-detector;1\0"
    /* 0x388a */ "@mozilla.org/thread-manager;1\0"
    /* 0x38a8 */ "@mozilla.org/preferences-service;1\0"
    /* 0x38cb */ "@mozilla.org/xpcom/version-comparator;1\0"
    /* 0x38f3 */ "@mozilla.org/clear-data-service;1\0"
    /* 0x3915 */ "@mozilla.org/presentation-device/multicastdns-provider;1\0"
    /* 0x394e */ "@mozilla.org/network/protocol/about;1?what=tabcrashed\0"
    /* 0x3984 */ "@mozilla.org/uriloader;1\0"
    /* 0x399d */ "@mozilla.org/nsClientAuthDialogs;1\0"
    /* 0x39c0 */ "@mozilla.org/streamconv;1?from=application/http-index-format&to=text/html\0"
    /* 0x3a0a */ "@mozilla.org/synth-voice-registry;1\0"
    /* 0x3a2e */ "@mozilla.org/thumbnails/pagethumbs-service;1\0"
    /* 0x3a5b */ "@mozilla.org/network/protocol/about;1?what=webrtc\0"
    /* 0x3a8d */ "@mozilla.org/jsinspector;1\0"
    /* 0x3aa8 */ "@mozilla.org/dom/slow-script-debug;1\0"
    /* 0x3acd */ "@mozilla.org/load-context-info-factory;1\0"
    /* 0x3af6 */ "@mozilla.org/security/nsCertTree;1\0"
    /* 0x3b19 */ "@mozilla.org/streamconv;1?from=application/pdf&to=*/*\0"
    /* 0x3b4f */ "@mozilla.org/supports-char;1\0"
    /* 0x3b6c */ "@mozilla.org/uriloader/content-handler;1?type=text/html\0"
    /* 0x3ba4 */ "@mozilla.org/content-dispatch-chooser;1\0"
    /* 0x3bcc */ "@mozilla.org/network/protocol;1?name=moz-gio\0"
    /* 0x3bf9 */ "@mozilla.org/dom/createofferrequest;1\0"
    /* 0x3c1f */ "@mozilla.org/network/input-stream-pump;1\0"
    /* 0x3c48 */ "@mozilla.org/nsTokenDialogs;1\0"
    /* 0x3c66 */ "@mozilla.org/websocketevent/service;1\0"
    /* 0x3c8c */ "@mozilla.org/browser/synced-bookmarks-merger;1\0"
    /* 0x3cbb */ "@mozilla.org/network/authprompt-adapter-factory;1\0"
    /* 0x3ced */ "@mozilla.org/gfx/devicecontextspec;1\0"
    /* 0x3d12 */ "@mozilla.org/network/protocol;1?name=about\0"
    /* 0x3d3d */ "@mozilla.org/embedcomp/prompt-collection;1\0"
    /* 0x3d68 */ "@mozilla.org/autocomplete/search;1?name=places-tag-autocomplete\0"
    /* 0x3da8 */ "@mozilla.org/nsTokenPasswordDialogs;1\0"
    /* 0x3dce */ "@mozilla.org/dom/rtpsender;1\0"
    /* 0x3deb */ "@mozilla.org/network/protocol/about;1?what=reader\0"
    /* 0x3e1d */ "@mozilla.org/network/protocol;1?name=jar\0"
    /* 0x3e46 */ "@mozilla.org/network/protocol/about;1?what=support\0"
    /* 0x3e79 */ "@mozilla.org/storagestream;1\0"
    /* 0x3e96 */ "@mozilla.org/addons/integration;1\0"
    /* 0x3eb8 */ "@mozilla.org/streamconv;1?from=multipart/x-mixed-replace&to=*/*\0"
    /* 0x3ef8 */ "@mozilla.org/network/protocol/about;1?what=newpreferences\0"
    /* 0x3f32 */ "@mozilla.org/applicationchooser;1\0"
    /* 0x3f54 */ "@mozilla.org/weave/service;1\0"
    /* 0x3f71 */ "@mozilla.org/network/protocol/about;1?what=blocked\0"
    /* 0x3fa4 */ "@mozilla.org/network/protocol;1?name=moz\0"
    /* 0x3fcd */ "@mozilla.org/network/protocol/about;1?what=buildconfig\0"
    /* 0x4004 */ "@mozilla.org/network/protocol/about;1?what=processes\0"
    /* 0x4039 */ "@mozilla.org/widget/dragservice;1\0"
    /* 0x405b */ "@mozilla.org/inspector/deep-tree-walker;1\0"
    /* 0x4085 */ "@mozilla.org/network/protocol/about;1?what=url-classifier\0"
    /* 0x40bf */ "@mozilla.org/consoleservice;1\0"
    /* 0x40dd */ "@mozilla.org/toolkit/URLFormatterService;1\0"
    /* 0x4108 */ "@mozilla.org/network/protocol/about;1?what=printpreview\0"
    /* 0x4140 */ "@mozilla.org/autocomplete/search;1?name=login-doorhanger-password\0"
    /* 0x4182 */ "@mozilla.org/network/incremental-download;1\0"
    /* 0x41ae */ "@mozilla.org/uriclassifierservice\0"
    /* 0x41d0 */ "@mozilla.org/hash-property-bag;1\0"
    /* 0x41f1 */ "@mozilla.org/url-classifier/channel-classifier-service;1\0"
    /* 0x422a */ "@mozilla.org/scriptable-content-iterator;1\0"
    /* 0x4255 */ "@mozilla.org/url-classifier/hashcompleter;1\0"
    /* 0x4281 */ "@mozilla.org/startupcacheinfo;1\0"
    /* 0x42a1 */ "@mozilla.org/push/Service;1\0"
    /* 0x42bd */ "@mozilla.org/security/x509certdb;1\0"
    /* 0x42e0 */ "@mozilla.org/nsGeneratingKeypairInfoDialogs;1\0"
    /* 0x430e */ "@mozilla.org/network/protocol/about;1?what=devtools\0"
    /* 0x4342 */ "@mozilla.org/sidebar;1\0"
    /* 0x4359 */ "@mozilla.org/uriloader/content-handler;1?type=text/css\0"
    /* 0x4390 */ "@mozilla.org/uriloader/content-handler;1?type=image/png\0"
    /* 0x43c8 */ "@mozilla.org/appshell/component/browser-status-filter;1\0"
    /* 0x4400 */ "@mozilla.org/push/PushManager;1\0"
    /* 0x4420 */ "@mozilla.org/network/protocol-proxy-service;1\0"
    /* 0x444e */ "@mozilla.org/network/protocol/about;1?what=srcdoc\0"
    /* 0x4480 */ "@mozilla.org/async-shutdown-service;1\0"
    /* 0x44a6 */ "@mozilla.org/permissionmanager;1\0"
    /* 0x44c7 */ "@mozilla.org/notificationStorage;1\0"
    /* 0x44ea */ "@mozilla.org/uriloader/external-helper-app-service;1\0"
    /* 0x451f */ "@mozilla.org/streamconv;1?from=deflate&to=uncompressed\0"
    /* 0x4556 */ "@mozilla.org/dom/rtcstatsreport;1\0"
    /* 0x4578 */ "@mozilla.org/nsCertificateDialogs;1\0"
    /* 0x459c */ "@mozilla.org/tools/profiler;1\0"
    /* 0x45ba */ "@mozilla.org/expandedprincipal;1\0"
    /* 0x45db */ "@mozilla.org/security/certstorage;1\0"
    /* 0x45ff */ "@mozilla.org/profile/migrator;1?app=browser&type=chromium\0"
    /* 0x4639 */ "@mozilla.org/libjar/zip-reader-cache;1\0"
    /* 0x4660 */ "@mozilla.org/focus-manager;1\0"
    /* 0x467d */ "@mozilla.org/content/plugin/document-loader-factory;1\0"
    /* 0x46b3 */ "@mozilla.org/intl/scriptableunicodeconverter\0"
    /* 0x46e0 */ "@mozilla.org/browser/nav-bookmarks-service;1\0"
    /* 0x470d */ "@mozilla.org/network/protocol/about;1?what=telemetry\0"
    /* 0x4742 */ "@mozilla.org/network/background-file-saver;1?mode=outputstream\0"
    /* 0x4781 */ "@mozilla.org/presentation/mockedsockettransport;1\0"
    /* 0x47b3 */ "@mozilla.org/network/protocol/about;1?what=logo\0"
    /* 0x47e3 */ "@mozilla.org/network/http-auth-manager;1\0"
    /* 0x480c */ "@mozilla.org/mediaManagerService;1\0"
    /* 0x482f */ "@mozilla.org/image/encoder;2?type=image/bmp\0"
    /* 0x485b */ "@mozilla.org/network/protocol/about;1?what=preferences\0"
    /* 0x4892 */ "@mozilla.org/toolkit/filewatcher/native-file-watcher;1\0"
    /* 0x48c9 */ "@mozilla.org/cspcontext;1\0"
    /* 0x48e3 */ "@mozilla.org/network/protocol;1?name=chrome\0"
    /* 0x490f */ "@mozilla.org/devicesensors;1\0"
    /* 0x492c */ "@mozilla.org/network/protocol;1?name=view-source\0"
    /* 0x495d */ "@mozilla.org/gfx/printerlist;1\0"
    /* 0x497c */ "@mozilla.org/dom/sdb-connection;1\0"
    /* 0x499e */ "@mozilla.org/process/util;1\0"
    /* 0x49ba */ "@mozilla.org/storage/activity-service;1\0"
    /* 0x49e2 */ "@mozilla.org/webspeech/service;1?name=online\0"
    /* 0x4a0f */ "@mozilla.org/pref-localizedstring;1\0"
    /* 0x4a33 */ "@mozilla.org/network/protocol/about;1?what=newtab\0"
    /* 0x4a65 */ "@mozilla.org/network/application-cache-service;1\0"
    /* 0x4a96 */ "@mozilla.org/url-classifier/dbservice;1\0"
    /* 0x4abe */ "@mozilla.org/network/protocol;1?name=moz-fonttable\0"
    /* 0x4af1 */ "@mozilla.org/presentation/datachanneltransport;1\0"
    /* 0x4b22 */ "@mozilla.org/network/protocol/about;1?what=ion\0"
    /* 0x4b51 */ "@mozilla.org/security/nssversion;1\0"
    /* 0x4b74 */ "@mozilla.org/network/protocol;1?name=indexeddb\0"
    /* 0x4ba3 */ "@mozilla.org/streamconv;1?from=uncompressed&to=rawdeflate\0"
    /* 0x4bdd */ "@mozilla.org/push/Notifier;1\0"
    /* 0x4bfa */ "@mozilla.org/jsctypes;1\0"
    /* 0x4c12 */ "@mozilla.org/base/telemetry-startup;1\0"
    /* 0x4c38 */ "@mozilla.org/dom/sessionStorage-manager;1\0"
    /* 0x4c62 */ "@mozilla.org/uriloader/content-handler;1?type=image/bmp\0"
    /* 0x4c9a */ "@mozilla.org/supports-PRTime;1\0"
    /* 0x4cb9 */ "@mozilla.org/txttohtmlconv;1\0"
    /* 0x4cd6 */ "@mozilla.org/parental-controls-service;1\0"
    /* 0x4cff */ "@mozilla.org/system-proxy-settings;1\0"
    /* 0x4d24 */ "@mozilla.org/supports-PRInt64;1\0"
    /* 0x4d44 */ "@mozilla.org/cookieService;1\0"
    /* 0x4d61 */ "@mozilla.org/gfx/fontenumerator;1\0"
    /* 0x4d83 */ "@mozilla.org/autocomplete/search;1?name=form-history\0"
    /* 0x4db8 */ "@mozilla.org/filepicker;1\0"
    /* 0x4dd2 */ "@mozilla.org/streamconv;1?from=application/vnd.mozilla.webext.unlocalized&to=text/css\0"
    /* 0x4e28 */ "@mozilla.org/streamconv;1?from=x-gzip&to=uncompressed\0"
    /* 0x4e5e */ "@mozilla.org/network/stream-transport-service;1\0"
    /* 0x4e8e */ "@mozilla.org/geolocation/mls-provider;1\0"
    /* 0x4eb6 */ "@mozilla.org/browser/search-service;1\0"
    /* 0x4edc */ "@mozilla.org/network/protocol/about;1?what=serviceworkers\0"
    /* 0x4f16 */ "@mozilla.org/appshell/window-mediator;1\0"
    /* 0x4f3e */ "@mozilla.org/streamconv;1?from=application/octet-stream&to=text/html\0"
    /* 0x4f83 */ "@mozilla.org/network/protocol/about;1?what=license\0"
    /* 0x4fb6 */ "@mozilla.org/network/protocol/about;1?what=crashes\0"
    /* 0x4fe9 */ "@mozilla.org/streamconv;1?from=x-compress&to=uncompressed\0"
    /* 0x5023 */ "@mozilla.org/accessibilityService;1\0"
    /* 0x5047 */ "@mozilla.org/browser/browserglue;1\0"
    /* 0x506a */ "@mozilla.org/toolkit/glean;1\0"
    /* 0x5087 */ "@mozilla.org/widget/htmlformatconverter;1\0"
    /* 0x50b1 */ "@mozilla.org/securityconsole/message;1\0"
    /* 0x50d8 */ "@mozilla.org/uriloader/content-handler;1?type=application/xhtml+xml\0"
    /* 0x511c */ "@mozilla.org/network/tls-server-socket;1\0"
    /* 0x5145 */ "@mozilla.org/network/default-uri-mutator;1\0"
    /* 0x5170 */ "@mozilla.org/security/osreauthenticator;1\0"
    /* 0x519a */ "@mozilla.org/offlinecacheupdate-service;1\0"
    /* 0x51c4 */ "@mozilla.org/uriloader/local-handler-app;1\0"
    /* 0x51ef */ "@mozilla.org/timer;1\0"
    /* 0x5204 */ "@mozilla.org/url-classifier/streamupdater;1\0"
    /* 0x5230 */ "@mozilla.org/network/protocol/about;1?what=compat\0"
    /* 0x5262 */ "@mozilla.org/image/tools;1\0"
    /* 0x527d */ "@mozilla.org/uriloader/content-handler;1?type=text/xml\0"
    /* 0x52b4 */ "@mozilla.org/streamconv;1?from=application/pdf&to=text/html\0"
    /* 0x52f0 */ "@mozilla.org/principal;1\0"
    /* 0x5309 */ "@mozilla.org/embedcomp/prompt-service;1\0"
    /* 0x5331 */ "@mozilla.org/image/encoder;2?type=image/jpeg\0"
    /* 0x535e */ "@mozilla.org/webnavigation-info;1\0"
    /* 0x5380 */ "@mozilla.org/supports-interface-pointer;1\0"
    /* 0x53aa */ "@mozilla.org/security/pk11tokendb;1\0"
    /* 0x53ce */ "@mozilla.org/network/protocol;1?name=resource\0"
    /* 0x53fc */ "@mozilla.org/network/protocol/about;1?what=studies\0"
    /* 0x542f */ "@mozilla.org/supports-PRInt32;1\0"
    /* 0x544f */ "@mozilla.org/parentprocessmessagemanager;1\0"
    /* 0x547a */ "@mozilla.org/security/transportsecurityinfo;1\0"
    /* 0x54a8 */ "@mozilla.org/network/protocol/about;1?what=crashcontent\0"
    /* 0x54e0 */ "@mozilla.org/network/protocol;1?name=ws\0"
    /* 0x5508 */ "@mozilla.org/prompter;1\0"
    /* 0x5520 */ "@mozilla.org/uriloader/content-handler;1?type=image/gif\0"
    /* 0x5558 */ "@mozilla.org/network/protocol/about;1?what=home\0"
    /* 0x5588 */ "@mozilla.org/network/url-parser;1?auth=yes\0"
    /* 0x55b3 */ "@mozilla.org/ipc/processselector;1\0"
    /* 0x55d6 */ "@mozilla.org/network/protocol/about;1?what=robots\0"
    /* 0x5608 */ "@mozilla.org/parserutils;1\0"
    /* 0x5623 */ "@mozilla.org/supports-cstring;1\0"
    /* 0x5643 */ "@mozilla.org/layout/contentserializer;1?mimetype=application/vnd.mozilla.xul+xml\0"
    /* 0x5694 */ "@mozilla.org/security/clientAuthRememberService;1\0"
    /* 0x56c6 */ "@mozilla.org/network/protocol/about;1?what=mozilla\0"
    /* 0x56f9 */ "@mozilla.org/childprocessmessagemanager;1\0"
    /* 0x5723 */ "@mozilla.org/widget/useridleservice;1\0"
    /* 0x5749 */ "@mozilla.org/login-manager/autocompletesearch;1\0"
    /* 0x5779 */ "@mozilla.org/network/protocol;1?name=https\0"
    /* 0x57a4 */ "@mozilla.org/tracking-db-service;1\0"
    /* 0x57c7 */ "@mozilla.org/network/protocol/about;1?what=addons\0"
    /* 0x57f9 */ "@mozilla.org/security/hmac;1\0"
    /* 0x5816 */ "@mozilla.org/profile/migrator;1?app=browser&type=chrome\0"
    /* 0x584e */ "@mozilla.org/login-manager/storage/json;1\0"
    /* 0x5878 */ "@mozilla.org/network/simple-uri-mutator;1\0"
    /* 0x58a2 */ "@mozilla.org/security/sdr;1\0"
    /* 0x58be */ "@mozilla.org/colorpicker;1\0"
    /* 0x58d9 */ "@mozilla.org/net/osfileconstantsservice;1\0"
    /* 0x5903 */ "@mozilla.org/widget/clipboardhelper;1\0"
    /* 0x5929 */ "@mozilla.org/webvttParserWrapper;1\0"
    /* 0x594c */ "@mozilla.org/login-manager/loginInfo;1\0"
    /* 0x5973 */ "@mozilla.org/uriloader/web-handler-app;1\0"
    /* 0x599c */ "@mozilla.org/network/protocol/about;1?what=plugins\0"
    /* 0x59cf */ "@mozilla.org/intl/texttosuburi;1\0"
    /* 0x59f0 */ "@mozilla.org/uriloader/content-handler;1?type=text/rdf\0"
    /* 0x5a27 */ "@mozilla.org/message-loop;1\0"
    /* 0x5a43 */ "@mozilla.org/presentation/datachanneltransportbuilder;1\0"
    /* 0x5a7b */ "@mozilla.org/dom/rtcicecandidate;1\0"
    /* 0x5a9e */ "@mozilla.org/peerconnection;1\0"
    /* 0x5abc */ "@mozilla.org/zipwriter;1\0"
    /* 0x5ad5 */ "@mozilla.org/network/protocol;1?name=data\0"
    /* 0x5aff */ "@mozilla.org/network/protocol/about;1?what=credits\0"
    /* 0x5b32 */ "@mozilla.org/uriloader/content-handler;1?type=image/jpeg\0"
    /* 0x5b6b */ "@mozilla.org/layout/contentserializer;1?mimetype=image/svg+xml\0"
    /* 0x5baa */ "@mozilla.org/network/tcp-filter-handler;1?name=stun\0"
    /* 0x5bde */ "@mozilla.org/network/simple-stream-listener;1\0"
    /* 0x5c0c */ "@mozilla.org/uriloader/content-handler;1?type=application/x-xpinstall\0"
    /* 0x5c52 */ "@mozilla.org/docshell/uri-fixup-info;1\0"
    /* 0x5c79 */ "@mozilla.org/network/protocol/about;1?what=welcome\0"
    /* 0x5cac */ "@mozilla.org/network/cache-storage-service;1\0"
    /* 0x5cd9 */ "@mozilla.org/globalmessagemanager;1\0"
    /* 0x5cfd */ "@mozilla.org/toolkit/shutdown-terminator;1\0"
    /* 0x5d28 */ "@mozilla.org/cookieJarSettings;1\0"
    /* 0x5d49 */ "@mozilla.org/libjar/zip-reader;1\0"
    /* 0x5d6a */ "@mozilla.org/network/standard-url-mutator;1\0"
    /* 0x5d96 */ "@mozilla.org/network/protocol/about;1?what=logins\0"
    /* 0x5dc8 */ "@mozilla.org/base/telemetry;1\0"
    /* 0x5de6 */ "@mozilla.org/prefetch-service;1\0"
    /* 0x5e06 */ "@mozilla.org/dom/browser-element-api;1\0"
    /* 0x5e2d */ "@mozilla.org/jsreflect;1\0"
    /* 0x5e46 */ "@mozilla.org/widget/appshell/gtk;1\0"
    /* 0x5e69 */ "@mozilla.org/supports-PRUint8;1\0"
    /* 0x5e89 */ "@mozilla.org/network/protocol;1?name=page-icon\0"
    /* 0x5eb8 */ "@mozilla.org/network/http-activity-distributor;1\0"
    /* 0x5ee9 */ "@mozilla.org/network/background-file-saver;1?mode=streamlistener\0"
    /* 0x5f2a */ "@mozilla.org/security/keyobject;1\0"
    /* 0x5f4c */ "@mozilla.org/supports-string;1\0"
    /* 0x5f6b */ "@mozilla.org/login-manager/authprompter;1\0"
    /* 0x5f95 */ "@mozilla.org/ospermissionrequest;1\0"
    /* 0x5fb8 */ "@mozilla.org/presentation/presentationtcpsessiontransport;1\0"
    /* 0x5ff4 */ "@mozilla.org/supports-PRUint64;1\0"
    /* 0x6015 */ "@mozilla.org/satchel/form-autocomplete;1\0"
    /* 0x603e */ "@mozilla.org/toolkit/profile-migrator;1\0"
    /* 0x6066 */ "@mozilla.org/transfer;1\0"
    /* 0x607e */ "@mozilla.org/cascade-filter;1\0"
    /* 0x609c */ "@mozilla.org/uriloader/content-handler;1?type=application/http-index-format\0"
    /* 0x60e8 */ "@mozilla.org/network/cache-service;1\0"
    /* 0x610d */ "@mozilla.org/streamconv;1?from=uncompressed&to=gzip\0"
    /* 0x6141 */ "@mozilla.org/network/file-output-stream;1\0"
    /* 0x616b */ "@mozilla.org/network/protocol/about;1?what=pocket-signup\0"
    /* 0x61a4 */ "@mozilla.org/eventsourceevent/service;1\0"
    /* 0x61cc */ "@mozilla.org/content/style-sheet-service;1\0"
    /* 0x61f7 */ "@mozilla.org/toolkit/components/mdnsresponder/dns-info;1\0"
    /* 0x6230 */ "@mozilla.org/network/protocol;1?name=dot\0"
    /* 0x6259 */ "@mozilla.org/security/keyobjectfactory;1\0"
    /* 0x6282 */ "@mozilla.org/sandbox/sandbox-settings;1\0"
    /* 0x62aa */ "@mozilla.org/streamConverters;1\0"
    /* 0x62ca */ "@mozilla.org/network/protocol;1?name=file\0"
    /* 0x62f4 */ "@mozilla.org/nullprincipal;1\0"
    /* 0x6311 */ "@mozilla.org/network/protocol;1?name=ftp\0"
    /* 0x633a */ "@mozilla.org/spellchecker/engine;1\0"
    /* 0x635d */ "@mozilla.org/addons/installtrigger;1\0"
    /* 0x6382 */ "@mozilla.org/widget/printdialog-service;1\0"
    /* 0x63ac */ "@mozilla.org/network/dns-service;1\0"
    /* 0x63cf */ "@mozilla.org/mime;1\0"
    /* 0x63e3 */ "@mozilla.org/toolkit/download-platform;1\0"
    /* 0x640c */ "@mozilla.org/io/string-input-stream;1\0"
    /* 0x6432 */ "@mozilla.org/layout/content-policy;1\0"
    /* 0x6457 */ "@mozilla.org/content/dropped-link-handler;1\0"
    /* 0x6483 */ "@mozilla.org/moz/jssubscript-loader;1\0"
    /* 0x64a9 */ "@mozilla.org/sound;1\0"
    /* 0x64be */ "@mozilla.org/network/protocol/about;1?what=policies\0"
    /* 0x64f2 */ "@mozilla.org/intl/converter-input-stream;1\0"
    /* 0x651d */ "@mozilla.org/helperapplauncherdialog;1\0"
    /* 0x6544 */ "@mozilla.org/browser/shell-service;1\0"
    /* 0x6569 */ "@mozilla.org/array;1\0"
    /* 0x657e */ "@mozilla.org/main-process-singleton;1\0"
    /* 0x65a4 */ "@mozilla.org/file/local;1\0"
    /* 0x65be */ "@mozilla.org/network/protocol/about;1?what=blank\0"
    /* 0x65ef */ "@mozilla.org/login-manager/storage/default;1\0"
    /* 0x661c */ "@mozilla.org/consoleAPI-storage;1\0"
    /* 0x663e */ "@mozilla.org/xpcom/memory-service;1\0"
    /* 0x6662 */ "@mozilla.org/xre/runtime;1\0"
    /* 0x667d */ "@mozilla.org/network/mime-input-stream;1\0"
    /* 0x66a6 */ "@mozilla.org/network/protocol/about;1?what=framecrashed\0"
    /* 0x66de */ "@mozilla.org/network/protocol;1?name=moz-icon\0"
    /* 0x670c */ "@mozilla.org/gecko-media-plugin-service;1\0"
    /* 0x6736 */ "@mozilla.org/network/file-input-stream;1\0"
    /* 0x675f */ "@mozilla.org/layout/contentserializer;1?mimetype=text/plain\0"
    /* 0x679b */ "@mozilla.org/wifi/monitor;1\0"
    /* 0x67b7 */ "@mozilla.org/uriloader/external-protocol-service;1\0"
    /* 0x67ea */ "@mozilla.org/widget/taskbarprogress/gtk;1\0"
    /* 0x6814 */ "@mozilla.org/addons/policy-service;1\0"
    /* 0x6839 */ "@mozilla.org/streamconv;1?from=multipart/byteranges&to=*/*\0"
    /* 0x6874 */ "@mozilla.org/layout/contentserializer;1?mimetype=application/xml\0"
    /* 0x68b5 */ "@mozilla.org/supports-PRBool;1\0"
    /* 0x68d4 */ "@mozilla.org/network/protocol/about;1?what=about\0"
    /* 0x6905 */ "@mozilla.org/uriloader/dbus-handler-app;1\0"
    /* 0x692f */ "@mozilla.org/streamconv;1?from=multipart/mixed&to=*/*\0"
    /* 0x6965 */ "@mozilla.org/io/multiplex-input-stream;1\0"
    /* 0x698e */ "@mozilla.org/updates/update-processor;1\0"
    /* 0x69b6 */ "@mozilla.org/security/contentsignatureverifier;1\0"
    /* 0x69e7 */ "@mozilla.org/autocomplete/search;1?name=unifiedcomplete\0"
    /* 0x6a1f */ "@mozilla.org/network/io-service;1\0"
    /* 0x6a41 */ "@mozilla.org/network/protocol/about;1?what=devtools-toolbox\0"
    /* 0x6a7d */ "@mozilla.org/services/settings;1\0"
    /* 0x6a9e */ "@mozilla.org/network/dashboard;1\0"
    /* 0x6abf */ "@mozilla.org/xpcom/debug;1\0"
    /* 0x6ada */ "@mozilla.org/variant;1\0"
    /* 0x6af1 */ "@mozilla.org/network/downloader;1\0"
    /* 0x6b13 */ "@mozilla.org/network/incremental-stream-loader;1\0"
    /* 0x6b44 */ "@mozilla.org/network/protocol/about;1?what=debugging\0"
    /* 0x6b79 */ "@mozilla.org/network/protocol/about;1?what=config\0"
    /* 0x6bab */ "@mozilla.org/login-manager/prompter;1\0"
    /* 0x6bd1 */ "@mozilla.org/security/local-cert-service;1\0"
    /* 0x6bfc */ "@mozilla.org/io-util;1\0"
    /* 0x6c13 */ "@mozilla.org/profile/migrator;1?app=browser&type=chrome-beta\0"
    /* 0x6c50 */ "@mozilla.org/scriptableinputstream;1\0"
    /* 0x6c75 */ "@mozilla.org/browser/history;1\0"
    /* 0x6c94 */ "@mozilla.org/supports-PRInt16;1\0"
    /* 0x6cb4 */ "@mozilla.org/scriptsecuritymanager;1\0"
    /* 0x6cd9 */ "@mozilla.org/mozintl;1\0"
    /* 0x6cf0 */ "@mozilla.org/enterprisepolicies;1\0"
    /* 0x6d12 */ "@mozilla.org/network/protocol/about;1?what=rights\0"
    /* 0x6d44 */ "@mozilla.org/toolkit/crash-reporter;1\0"
    /* 0x6d6a */ "@mozilla.org/network/protocol/about;1?what=sessionrestore\0"
    /* 0x6da4 */ "@mozilla.org/contentsecuritymanager;1\0"
    /* 0x6dca */ "@mozilla.org/permissiondelegatehandler;1\0"
    /* 0x6df3 */ "@mozilla.org/pipe;1\0"
    /* 0x6e07 */ "@mozilla.org/login-manager;1\0"
    /* 0x6e24 */ "@mozilla.org/toolkit/viaduct;1\0"
    /* 0x6e43 */ "@mozilla.org/network/input-stream-channel;1\0"
    /* 0x6e6f */ "@mozilla.org/gio-service;1\0"
    /* 0x6e8a */ "@mozilla.org/scripterror;1\0"
    /* 0x6ea5 */ "@mozilla.org/network/protocol/about;1?what=sync-log\0"
    /* 0x6ed9 */ "@mozilla.org/passwordmanager/authpromptfactory;1\0"
    /* 0x6f0a */ "@mozilla.org/intl/collation-factory;1\0"
    /* 0x6f30 */ "@mozilla.org/uriloader/content-handler;1?type=text/plain\0"
    /* 0x6f69 */ "@mozilla.org/embeddor.implemented/web-protocol-handler-registrar;1\0"
    /* 0x6fac */ "@mozilla.org/binaryinputstream;1\0"
    /* 0x6fcd */ "@mozilla.org/content/html-menu-builder;1\0"
    /* 0x6ff6 */ "@mozilla.org/network/protocol;1?name=moz-safe-about\0"
    /* 0x702a */ "@mozilla.org/plugin/host;1\0"
    /* 0x7045 */ "@mozilla.org/file/directory_service;1\0"
    /* 0x706b */ "@mozilla.org/mozintlhelper;1\0"
    /* 0x7088 */ "@mozilla.org/network/protocol/about;1?what=cache-entry\0"
    /* 0x70bf */ "@mozilla.org/extensions/storage/sync;1\0"
    /* 0x70e6 */ "@mozilla.org/network/stream-listener-tee;1\0"
    /* 0x7111 */ "@mozilla.org/categorymanager;1\0"
    /* 0x7130 */ "@mozilla.org/dom/peerconnection;1\0"
    /* 0x7152 */ "@mozilla.org/network/safe-file-output-stream;1\0"
    /* 0x7181 */ "@mozilla.org/network/protocol/about;1?what=newinstall\0"
    /* 0x71b7 */ "@mozilla.org/network/protocol/about;1?what=neterror\0"
    /* 0x71eb */ "@mozilla.org/network/predictor;1\0"
    /* 0x720c */ "@mozilla.org/network/protocol;1?name=wss\0"
    /* 0x7235 */ "@mozilla.org/network/protocol/about;1?what=httpsonlyerror\0"
    /* 0x726f */ "@mozilla.org/sandbox/syscall-reporter;1\0"
    /* 0x7297 */ "@mozilla.org/updates/update-service;1\0"
    /* 0x72bd */ "@mozilla.org/reputationservice/login-reputation-service;1\0"
    /* 0x72f7 */ "@mozilla.org/network/captive-portal-service;1\0"
    /* 0x7325 */ "@mozilla.org/network/protocol/about;1?what=privatebrowsing\0"
    /* 0x7360 */ "@mozilla.org/text-input-processor;1\0"
    /* 0x7384 */ "@mozilla.org/network/protocol/about;1?what=crashparent\0"
    /* 0x73bb */ "@mozilla.org/uuid-generator;1\0"
    /* 0x73d9 */ "@mozilla.org/satchel/inputlist-autocomplete;1\0"
    /* 0x7407 */ "@mozilla.org/network/protocol/about;1?what=welcomeback\0"
    /* 0x743e */ "@mozilla.org/security/pkcs11moduledb;1\0"
    /* 0x7465 */ "@mozilla.org/network/protocol/about;1?what=performance\0"
    /* 0x749c */ "@mozilla.org/tracking-url-decoration-service;1\0"
    /* 0x74cb */ "@mozilla.org/uriloader/content-handler;1?type=image/jpg\0"
    /* 0x7503 */ "@mozilla.org/url-classifier/prefixset;1\0"
    /* 0x752b */ "@mozilla.org/network/protocol/about;1?what=restartrequired\0"
    /* 0x7566 */ "@mozilla.org/webspeech/service;1?name=fake\0"
    /* 0x7591 */ "@mozilla.org/browser/aboutnewtab-service;1\0"
    /* 0x75bc */ "@mozilla.org/image/encoder;2?type=image/png\0"
    /* 0x75e8 */ "@mozilla.org/systemprincipal;1\0"
    /* 0x7607 */ "@mozilla.org/nss_errors_service;1\0"
    /* 0x7629 */ "@mozilla.org/storage/service;1\0"
    /* 0x7648 */ "@mozilla.org/cookiemanager;1\0"
    /* 0x7665 */ "@mozilla.org/uriloader/content-handler;1?type=image/vnd.microsoft.icon\0"
    /* 0x76ac */ "@mozilla.org/addons/addon-manager-startup;1\0"
    /* 0x76d8 */ "@mozilla.org/network/protocol/about;1?what=profiles\0"
    /* 0x770c */ "@mozilla.org/network/buffered-input-stream;1\0"
    /* 0x7739 */ "@mozilla.org/network/protocol/about;1?what=profiling\0"
    /* 0x776e */ "@mozilla.org/dom/peerconnectionmanager;1\0"
    /* 0x7797 */ "@mozilla.org/network/protocol;1?name=moz-anno\0"
    /* 0x77c5 */ "@mozilla.org/extensions/blocklist;1\0"
    /* 0x77e9 */ "@mozilla.org/content-pref/service;1\0"
    /* 0x780d */ "@mozilla.org/network/native-dns-override;1\0"
    /* 0x7838 */ "@mozilla.org/psm;1\0"
    /* 0x784b */ "@mozilla.org/network/protocol/about;1?what=cache\0"
    /* 0x787c */ "@mozilla.org/network/protocol;1?name=moz-extension\0"
    /* 0x78af */ "@mozilla.org/embedding/browser/nsWebBrowserPersist;1\0"
    /* 0x78e4 */ "@mozilla.org/network/protocol/about;1?what=downloads\0"
    /* 0x7919 */ "@mozilla.org/image/encoder;2?type=image/vnd.microsoft.icon\0"
    /* 0x7954 */ "@mozilla.org/network/protocol/about;1?what=networking\0"
    /* 0x798a */ "@mozilla.org/docshell/uri-fixup;1\0"
    /* 0x79ac */ "@mozilla.org/spellchecker/personaldictionary;1\0"
    /* 0x79db */ "@mozilla.org/image/request;1\0"
    /* 0x79f8 */ "@mozilla.org/network/load-group;1\0"
    /* 0x7a1a */ "profiler\0"
    /* 0x7a23 */ "uriFixup\0"
    /* 0x7a2c */ "prompt\0"
    /* 0x7a33 */ "textToSubURI\0"
    /* 0x7a40 */ "cookies\0"
    /* 0x7a48 */ "io\0"
    /* 0x7a4b */ "tm\0"
    /* 0x7a4e */ "prefs\0"
    /* 0x7a54 */ "strings\0"
    /* 0x7a5c */ "els\0"
    /* 0x7a60 */ "storage\0"
    /* 0x7a68 */ "eTLD\0"
    /* 0x7a6d */ "perms\0"
    /* 0x7a73 */ "catMan\0"
    /* 0x7a7a */ "scriptSecurityManager\0"
    /* 0x7a90 */ "focus\0"
    /* 0x7a96 */ "cpmm\0"
    /* 0x7a9b */ "dirsvc\0"
    /* 0x7aa2 */ "blocklist\0"
    /* 0x7aac */ "domStorageManager\0"
    /* 0x7abe */ "locale\0"
    /* 0x7ac5 */ "startup\0"
    /* 0x7acd */ "console\0"
    /* 0x7ad5 */ "urlFormatter\0"
    /* 0x7ae2 */ "loadContextInfo\0"
    /* 0x7af2 */ "scriptloader\0"
    /* 0x7aff */ "policies\0"
    /* 0x7b08 */ "mm\0"
    /* 0x7b0b */ "qms\0"
    /* 0x7b0f */ "wm\0"
    /* 0x7b12 */ "vc\0"
    /* 0x7b15 */ "logins\0"
    /* 0x7b1c */ "ppmm\0"
    /* 0x7b21 */ "telemetry\0"
    /* 0x7b2b */ "appShell\0"
    /* 0x7b34 */ "crashmanager\0"
    /* 0x7b41 */ "sysinfo\0"
    /* 0x7b49 */ "cache2\0"
    /* 0x7b50 */ "xulStore\0"
    /* 0x7b59 */ "droppedLinkHandler\0"
    /* 0x7b6c */ "DOMRequest\0"
    /* 0x7b77 */ "appinfo\0"
    /* 0x7b7f */ "intl\0"
    /* 0x7b84 */ "clearData\0"
    /* 0x7b8e */ "clipboard\0"
    /* 0x7b98 */ "search\0"
    /* 0x7b9f */ "obs\0"
    /* 0x7ba3 */ "ww\0"
  "";

const StaticCategory gStaticCategories[kStaticCategoryCount] = {
  { { 0x0 } /* "Gecko-Content-Viewers" */,
    0, 35 },
  { { 0x2ae } /* "app-startup" */,
    35, 2 },
  { { 0x345 } /* "command-line-handler" */,
    37, 6 },
  { { 0x49a } /* "command-line-validator" */,
    43, 1 },
  { { 0x4bb } /* "content-policy" */,
    44, 7 },
  { { 0x5ee } /* "content-sniffing-services" */,
    51, 2 },
  { { 0x65c } /* "idle-daily" */,
    53, 4 },
  { { 0x77e } /* "net-channel-event-sinks" */,
    57, 2 },
  { { 0x796 } /* "net-content-sniffers" */,
    59, 2 },
  { { 0x7dd } /* "payment-request" */,
    61, 10 },
  { { 0xafe } /* "places-init-complete" */,
    71, 1 },
  { { 0xb47 } /* "pref-config-startup" */,
    72, 1 },
  { { 0xb87 } /* "profile-after-change" */,
    73, 9 },
  { { 0xdbb } /* "push" */,
    82, 1 },
  { { 0xdfa } /* "speech-synth-started" */,
    83, 2 },
  { { 0xe7d } /* "vacuum-participant" */,
    85, 1 },
  { { 0xec2 } /* "xpcom-startup" */,
    86, 2 },
};
const StaticCategoryEntry gStaticCategoryEntries[] = {
  /* "Gecko-Content-Viewers" */
  { { 0x16 } /* "application/ecmascript" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x5c } /* "application/http-index-format" */,
    { 0x7a } /* "@mozilla.org/content-viewers/http-index-format" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xa9 } /* "application/javascript" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xc0 } /* "application/json" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xd1 } /* "application/mathml+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xe8 } /* "application/rdf+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xfc } /* "application/vnd.mozilla.xul+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x11c } /* "application/vnd.wap.xhtml+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x13a } /* "application/x-javascript" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x153 } /* "application/x-view-source" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x16d } /* "application/xhtml+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x183 } /* "application/xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x193 } /* "image/apng" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x19e } /* "image/bmp" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1a8 } /* "image/gif" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1b2 } /* "image/icon" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1bd } /* "image/jpeg" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1c8 } /* "image/jpg" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1d2 } /* "image/pjpeg" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1de } /* "image/png" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1e8 } /* "image/svg+xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x1f6 } /* "image/vnd.microsoft.icon" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x20f } /* "image/x-icon" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x21c } /* "image/x-ms-bmp" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x22b } /* "image/x-png" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x237 } /* "text/cache-manifest" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x24b } /* "text/css" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x254 } /* "text/ecmascript" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x264 } /* "text/html" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x26e } /* "text/javascript" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x27e } /* "text/json" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x288 } /* "text/plain" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x293 } /* "text/rdf" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x29c } /* "text/vtt" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x2a5 } /* "text/xml" */,
    { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "app-startup" */
  { { 0x2ba } /* "FormFillController" */,
    { 0x2cd } /* "@mozilla.org/satchel/form-fill-controller;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x2f9 } /* "TelemetryControllerContent" */,
    { 0x314 } /* "@mozilla.org/base/telemetry-controller-content;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::CONTENT_PROCESS_ONLY },

  /* "command-line-handler" */
  { { 0x35a } /* "m-browser" */,
    { 0x364 } /* "@mozilla.org/browser/clh;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },
  { { 0x37f } /* "m-devtools" */,
    { 0x38a } /* "@mozilla.org/devtools/startup-clh;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x3ae } /* "m-recording" */,
    { 0x3ba } /* "@mozilla.org/commandlinehandler/general-startup;1?type=recording" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x3fb } /* "m-remote" */,
    { 0x404 } /* "@mozilla.org/commandlinehandler/general-startup;1?type=remote" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x442 } /* "x-default" */,
    { 0x44c } /* "@mozilla.org/browser/final-clh;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },
  { { 0x46d } /* "y-default" */,
    { 0x477 } /* "@mozilla.org/toolkit/default-clh;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "command-line-validator" */
  { { 0x4b1 } /* "b-browser" */,
    { 0x364 } /* "@mozilla.org/browser/clh;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },

  /* "content-policy" */
  { { 0x4ca } /* "@mozilla.org/addons/content-policy;1" */,
    { 0x4ca } /* "@mozilla.org/addons/content-policy;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x4ef } /* "@mozilla.org/data-document-content-policy;1" */,
    { 0x4ef } /* "@mozilla.org/data-document-content-policy;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x51b } /* "@mozilla.org/embedding/browser/content-policy;1" */,
    { 0x51b } /* "@mozilla.org/embedding/browser/content-policy;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x54b } /* "@mozilla.org/image-blocker-content-policy;1" */,
    { 0x54b } /* "@mozilla.org/image-blocker-content-policy;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x577 } /* "@mozilla.org/mixedcontentblocker;1" */,
    { 0x577 } /* "@mozilla.org/mixedcontentblocker;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x59a } /* "@mozilla.org/no-data-protocol-content-policy;1" */,
    { 0x59a } /* "@mozilla.org/no-data-protocol-content-policy;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x5c9 } /* "CSPService" */,
    { 0x5d4 } /* "@mozilla.org/cspservice;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "content-sniffing-services" */
  { { 0x608 } /* "@mozilla.org/image/loader;1" */,
    { 0x624 } /* "@mozilla.org/image/cache;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x63f } /* "@mozilla.org/media/sniffer;1" */,
    { 0x63f } /* "@mozilla.org/media/sniffer;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "idle-daily" */
  { { 0x667 } /* "ContentBlockingTelemetryService" */,
    { 0x687 } /* "@mozilla.org/content-blocking-telemetry-service;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },
  { { 0x6b9 } /* "MozStorage Vacuum Manager" */,
    { 0x6d3 } /* "@mozilla.org/storage/vacuum;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x6f1 } /* "PlacesDBUtilsIdleMaintenance" */,
    { 0x70e } /* "@mozilla.org/places/databaseUtilsIdleMaintenance;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x741 } /* "QuotaManagerService" */,
    { 0x755 } /* "@mozilla.org/dom/quota-manager-service;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "net-channel-event-sinks" */
  { { 0x577 } /* "@mozilla.org/mixedcontentblocker;1" */,
    { 0x577 } /* "@mozilla.org/mixedcontentblocker;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x5c9 } /* "CSPService" */,
    { 0x5d4 } /* "@mozilla.org/cspservice;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "net-content-sniffers" */
  { { 0x63f } /* "@mozilla.org/media/sniffer;1" */,
    { 0x63f } /* "@mozilla.org/media/sniffer;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x7ab } /* "JSONView" */,
    { 0x7b4 } /* "@mozilla.org/devtools/jsonview-sniffer;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "payment-request" */
  { { 0x7ed } /* "BasicCardMethodChangeDetails" */,
    { 0x80a } /* "@mozilla.org/dom/payments/basiccard-change-details;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x83f } /* "BasicCardResponseData" */,
    { 0x855 } /* "@mozilla.org/dom/payments/basiccard-response-data;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x889 } /* "GeneralMethodChangeDetails" */,
    { 0x8a4 } /* "@mozilla.org/dom/payments/general-change-details;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x8d7 } /* "GeneralResponseData" */,
    { 0x8eb } /* "@mozilla.org/dom/payments/general-response-data;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x91d } /* "PaymentAbortActionResponse" */,
    { 0x938 } /* "@mozilla.org/dom/payments/payment-abort-action-response;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x972 } /* "PaymentAddress" */,
    { 0x981 } /* "@mozilla.org/dom/payments/payment-address;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0x9ad } /* "PaymentCanMakeActionResponse" */,
    { 0x9ca } /* "@mozilla.org/dom/payments/payment-canmake-action-response;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xa06 } /* "PaymentCompleteActionResponse" */,
    { 0xa24 } /* "@mozilla.org/dom/payments/payment-complete-action-response;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xa61 } /* "PaymentRequestService" */,
    { 0xa77 } /* "@mozilla.org/dom/payments/payment-request-service;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xaab } /* "PaymentShowActionResponse" */,
    { 0xac5 } /* "@mozilla.org/dom/payments/payment-show-action-response;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "places-init-complete" */
  { { 0xb13 } /* "nsPlacesExpiration" */,
    { 0xb26 } /* "@mozilla.org/places/expiration;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "pref-config-startup" */
  { { 0xb5b } /* "ReadConfig Module" */,
    { 0xb6d } /* "@mozilla.org/readconfig;1" */,
    Module::BackgroundTasksSelector::ALL_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "profile-after-change" */
  { { 0xb9c } /* "CrashMonitor" */,
    { 0xba9 } /* "@mozilla.org/toolkit/crashmonitor;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xbcd } /* "CrashService" */,
    { 0xbda } /* "@mozilla.org/crashservice;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xbf6 } /* "PresentationDeviceManager" */,
    { 0xc10 } /* "@mozilla.org/presentation-device/manager;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xc3b } /* "PresentationService" */,
    { 0xc4f } /* "@mozilla.org/presentation/presentationservice;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xc7f } /* "PurgeTrackerService" */,
    { 0xc93 } /* "@mozilla.org/purge-tracker-service;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xcb8 } /* "formHistoryStartup" */,
    { 0xccb } /* "@mozilla.org/satchel/form-history-startup;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xcf7 } /* "nsTerminatorTelemetry" */,
    { 0xd0d } /* "@mozilla.org/toolkit/shutdown-terminator-telemetry;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xd42 } /* "nsUpdateServiceStub" */,
    { 0xd56 } /* "@mozilla.org/updates/update-service-stub;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xd81 } /* "nsUpdateTimerManager" */,
    { 0xd96 } /* "@mozilla.org/updates/timer-manager;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "push" */
  { { 0xdc0 } /* "chrome://fxa-device-update" */,
    { 0xddb } /* "@mozilla.org/fxaccounts/push;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },

  /* "speech-synth-started" */
  { { 0xe0f } /* "Fake Speech Synth" */,
    { 0xe21 } /* "@mozilla.org/fakesynth;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },
  { { 0xe3a } /* "SpeechDispatcher Speech Synth" */,
    { 0xe58 } /* "@mozilla.org/synthspeechdispatcher;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "vacuum-participant" */
  { { 0xe90 } /* "Places" */,
    { 0xe97 } /* "@mozilla.org/browser/nav-history-service;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::ANY_PROCESS },

  /* "xpcom-startup" */
  { { 0xed0 } /* "Fork Server Launcher" */,
    { 0xee5 } /* "@mozilla.org/fork-server-launcher;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },
  { { 0xf09 } /* "nsUpdateSyncManager" */,
    { 0xf1d } /* "@mozilla.org/updates/update-sync-manager;1" */,
    Module::BackgroundTasksSelector::NO_TASKS,
    Module::ProcessSelector::MAIN_PROCESS_ONLY },
};

const nsXPTInterface gInterfaces[] = {
  nsXPTInterface::nsIDroppedLinkHandler,
  nsXPTInterface::nsIPermissionManager,
  nsXPTInterface::nsIStringBundleService,
  nsXPTInterface::mozILocaleService,
  nsXPTInterface::nsITextToSubURI,
  nsXPTInterface::mozIJSSubScriptLoader,
  nsXPTInterface::nsIPrefService,
  nsXPTInterface::nsIPrefBranch,
  nsXPTInterface::nsILoadContextInfoFactory,
  nsXPTInterface::nsICacheStorageService,
  nsXPTInterface::nsIEffectiveTLDService,
  nsXPTInterface::nsIIOService,
  nsXPTInterface::nsISpeculativeConnect,
  nsXPTInterface::nsINetUtil,
  nsXPTInterface::nsICookieManager,
  nsXPTInterface::mozIStorageService,
  nsXPTInterface::nsIAppStartup,
  nsXPTInterface::nsIClearDataService,
  nsXPTInterface::nsIEnterprisePolicies,
  nsXPTInterface::mozIMozIntl,
  nsXPTInterface::nsILoginManager,
  nsXPTInterface::nsIPromptService,
  nsXPTInterface::nsISearchService,
  nsXPTInterface::nsITelemetry,
  nsXPTInterface::nsIURLFormatter,
  nsXPTInterface::nsIXULStore,
  nsXPTInterface::nsIXULRuntime,
  nsXPTInterface::nsIXULAppInfo,
  nsXPTInterface::nsIWindowWatcher,
  nsXPTInterface::nsIProfiler,
  nsXPTInterface::nsIClipboard,
  nsXPTInterface::nsIConsoleService,
  nsXPTInterface::nsIPropertyBag2,
  nsXPTInterface::nsISystemInfo,
  nsXPTInterface::nsIThreadManager,
  nsXPTInterface::nsIVersionComparator,
  nsXPTInterface::nsICategoryManager,
  nsXPTInterface::nsIObserverService,
  nsXPTInterface::nsIDirectoryService,
  nsXPTInterface::nsIProperties,
  nsXPTInterface::nsIAppShellService,
  nsXPTInterface::nsIWindowMediator,
  nsXPTInterface::nsIDOMRequestService,
  nsXPTInterface::nsIDOMStorageManager,
  nsXPTInterface::nsILocalStorageManager,
  nsXPTInterface::nsIQuotaManagerService,
  nsXPTInterface::nsIEventListenerService,
  nsXPTInterface::nsIFocusManager,
  nsXPTInterface::nsIScriptSecurityManager,
};

const StringOffset gComponentJSMs[] = {
 { 0xf48 } /* "chrome://remote/content/components/RemoteAgent.jsm" */,
 { 0xf7b } /* "resource:///modules/AboutDebuggingRegistration.jsm" */,
 { 0xfae } /* "resource:///modules/AboutDevToolsRegistration.jsm" */,
 { 0xfe0 } /* "resource:///modules/AboutDevToolsToolboxRegistration.jsm" */,
 { 0x1019 } /* "resource:///modules/AboutNewTabService.jsm" */,
 { 0x1044 } /* "resource:///modules/BrowserContentHandler.jsm" */,
 { 0x1072 } /* "resource:///modules/BrowserGlue.jsm" */,
 { 0x1096 } /* "resource:///modules/ChromeProfileMigrator.jsm" */,
 { 0x10c4 } /* "resource:///modules/DevToolsStartup.jsm" */,
 { 0x10ec } /* "resource:///modules/FirefoxProfileMigrator.jsm" */,
 { 0x111b } /* "resource:///modules/ProfileMigrator.jsm" */,
 { 0x1143 } /* "resource:///modules/PromptCollection.jsm" */,
 { 0x116c } /* "resource:///modules/WebProtocolHandlerRegistrar.jsm" */,
 { 0x11a0 } /* "resource://devtools/client/jsonview/Converter.jsm" */,
 { 0x11d2 } /* "resource://devtools/client/jsonview/Sniffer.jsm" */,
 { 0x1202 } /* "resource://gre/modules/AutoCompleteSimpleSearch.jsm" */,
 { 0x1236 } /* "resource://gre/modules/BrowserElementParent.jsm" */,
 { 0x1266 } /* "resource://gre/modules/CaptiveDetect.jsm" */,
 { 0x128f } /* "resource://gre/modules/ClearDataService.jsm" */,
 { 0x12bb } /* "resource://gre/modules/ConsoleAPIStorage.jsm" */,
 { 0x12e8 } /* "resource://gre/modules/ContentAreaDropListener.jsm" */,
 { 0x131b } /* "resource://gre/modules/ContentDispatchChooser.jsm" */,
 { 0x134d } /* "resource://gre/modules/ContentPrefService2.jsm" */,
 { 0x137c } /* "resource://gre/modules/CrashManager.jsm" */,
 { 0x13a4 } /* "resource://gre/modules/CrashService.jsm" */,
 { 0x13cc } /* "resource://gre/modules/DNSServiceDiscovery.jsm" */,
 { 0x13fb } /* "resource://gre/modules/DefaultCLH.jsm" */,
 { 0x1421 } /* "resource://gre/modules/DotProtocolHandler.jsm" */,
 { 0x144f } /* "resource://gre/modules/DownloadLegacy.jsm" */,
 { 0x1479 } /* "resource://gre/modules/EnterprisePolicies.jsm" */,
 { 0x14a7 } /* "resource://gre/modules/ExtensionStorageComponents.jsm" */,
 { 0x14dd } /* "resource://gre/modules/FormAutoComplete.jsm" */,
 { 0x1509 } /* "resource://gre/modules/FormHistoryStartup.jsm" */,
 { 0x1537 } /* "resource://gre/modules/FxAccountsPush.jsm" */,
 { 0x1561 } /* "resource://gre/modules/HTMLMenuBuilder.jsm" */,
 { 0x158c } /* "resource://gre/modules/HelperAppDlg.jsm" */,
 { 0x15b4 } /* "resource://gre/modules/InputListAutoComplete.jsm" */,
 { 0x15e5 } /* "resource://gre/modules/LoginAutoComplete.jsm" */,
 { 0x1612 } /* "resource://gre/modules/LoginInfo.jsm" */,
 { 0x1637 } /* "resource://gre/modules/LoginManager.jsm" */,
 { 0x165f } /* "resource://gre/modules/LoginManagerAuthPrompter.jsm" */,
 { 0x1693 } /* "resource://gre/modules/LoginManagerPrompter.jsm" */,
 { 0x16c3 } /* "resource://gre/modules/MainProcessSingleton.jsm" */,
 { 0x16f3 } /* "resource://gre/modules/MozProtocolHandler.jsm" */,
 { 0x1721 } /* "resource://gre/modules/NetworkGeolocationProvider.jsm" */,
 { 0x1757 } /* "resource://gre/modules/NotificationStorage.jsm" */,
 { 0x1786 } /* "resource://gre/modules/PageThumbsStorageService.jsm" */,
 { 0x17ba } /* "resource://gre/modules/PartitioningExceptionListService.jsm" */,
 { 0x17f6 } /* "resource://gre/modules/PlacesDBUtils.jsm" */,
 { 0x181f } /* "resource://gre/modules/PlacesExpiration.jsm" */,
 { 0x184b } /* "resource://gre/modules/PresentationControlService.jsm" */,
 { 0x1881 } /* "resource://gre/modules/PresentationDataChannelSessionTransport.jsm" */,
 { 0x18c4 } /* "resource://gre/modules/ProcessSelector.jsm" */,
 { 0x18ef } /* "resource://gre/modules/Prompter.jsm" */,
 { 0x1913 } /* "resource://gre/modules/PurgeTrackerService.jsm" */,
 { 0x1942 } /* "resource://gre/modules/Push.jsm" */,
 { 0x1962 } /* "resource://gre/modules/PushComponents.jsm" */,
 { 0x198c } /* "resource://gre/modules/RecordingCmdLine.jsm" */,
 { 0x19b8 } /* "resource://gre/modules/SearchService.jsm" */,
 { 0x19e1 } /* "resource://gre/modules/SearchSuggestions.jsm" */,
 { 0x1a0e } /* "resource://gre/modules/ShieldContentProcess.jsm" */,
 { 0x1a3e } /* "resource://gre/modules/Sidebar.jsm" */,
 { 0x1a61 } /* "resource://gre/modules/SimpleServices.jsm" */,
 { 0x1a8b } /* "resource://gre/modules/SlowScriptDebug.jsm" */,
 { 0x1ab6 } /* "resource://gre/modules/TaggingService.jsm" */,
 { 0x1ae0 } /* "resource://gre/modules/TelemetryControllerContent.jsm" */,
 { 0x1b16 } /* "resource://gre/modules/TelemetryStartup.jsm" */,
 { 0x1b42 } /* "resource://gre/modules/TerminatorTelemetry.jsm" */,
 { 0x1b71 } /* "resource://gre/modules/TooltipTextProvider.jsm" */,
 { 0x1ba0 } /* "resource://gre/modules/TrackingDBService.jsm" */,
 { 0x1bcd } /* "resource://gre/modules/URIFixup.jsm" */,
 { 0x1bf1 } /* "resource://gre/modules/URLDecorationAnnotationsService.jsm" */,
 { 0x1c2c } /* "resource://gre/modules/URLFormatter.jsm" */,
 { 0x1c54 } /* "resource://gre/modules/UnifiedComplete.jsm" */,
 { 0x1c7f } /* "resource://gre/modules/UpdateService.jsm" */,
 { 0x1ca8 } /* "resource://gre/modules/UpdateServiceStub.jsm" */,
 { 0x1cd5 } /* "resource://gre/modules/UpdateTimerManager.jsm" */,
 { 0x1d03 } /* "resource://gre/modules/UrlClassifierExceptionListService.jsm" */,
 { 0x1d40 } /* "resource://gre/modules/UrlClassifierHashCompleter.jsm" */,
 { 0x1d76 } /* "resource://gre/modules/UrlClassifierLib.jsm" */,
 { 0x1da2 } /* "resource://gre/modules/UrlClassifierListManager.jsm" */,
 { 0x1dd6 } /* "resource://gre/modules/WebHandlerApp.jsm" */,
 { 0x1dff } /* "resource://gre/modules/WebVTTParserWrapper.jsm" */,
 { 0x1e2e } /* "resource://gre/modules/WellKnownOpportunisticUtils.jsm" */,
 { 0x1e65 } /* "resource://gre/modules/XULStore.jsm" */,
 { 0x1e89 } /* "resource://gre/modules/addonManager.js" */,
 { 0x1eb0 } /* "resource://gre/modules/amContentHandler.jsm" */,
 { 0x1edc } /* "resource://gre/modules/amInstallTrigger.jsm" */,
 { 0x1f08 } /* "resource://gre/modules/amWebAPI.jsm" */,
 { 0x1f2c } /* "resource://gre/modules/crypto-SDR.js" */,
 { 0x1f51 } /* "resource://gre/modules/media/PeerConnection.jsm" */,
 { 0x1f81 } /* "resource://gre/modules/mozIntl.jsm" */,
 { 0x1fa4 } /* "resource://gre/modules/nsAsyncShutdown.jsm" */,
 { 0x1fcf } /* "resource://gre/modules/nsCrashMonitor.jsm" */,
 { 0x1ff9 } /* "resource://gre/modules/pdfjs.js" */,
 { 0x2019 } /* "resource://gre/modules/storage-json.js" */,
 { 0x2040 } /* "resource://services-settings/RemoteSettingsComponents.jsm" */,
 { 0x207a } /* "resource://services-sync/Weave.jsm" */,
 { 0x209d } /* "resource://webcompat/AboutCompat.jsm" */,
};

/**
 * Returns a nsCString corresponding to the given entry in the `gStrings` string
 * table. The resulting nsCString points directly to static storage, and does
 * not incur any memory allocation overhead.
 */
static inline nsCString GetString(const StringOffset& aOffset) {
  const char* str = &gStrings[aOffset.mOffset];
  nsCString result;
  result.AssignLiteral(str, strlen(str));
  return result;
}

nsCString ContractEntry::ContractID() const {
  return GetString(mContractID);
}

bool ContractEntry::Matches(const nsACString& aContractID) const {
  return aContractID == ContractID() && Module().Active();
}


static nsresult ConstructJSMComponent(const nsACString& aURI,
                                      const char* aConstructor,
                                      nsISupports** aResult) {
  if (!nsComponentManagerImpl::JSLoaderReady()) {
    return NS_ERROR_NOT_AVAILABLE;
  }

  AutoJSAPI jsapi;
  MOZ_ALWAYS_TRUE(jsapi.Init(xpc::PrivilegedJunkScope()));
  JSContext* cx = jsapi.cx();

  JS::RootedObject global(cx);
  JS::RootedObject exports(cx);
  MOZ_TRY(mozJSComponentLoader::Get()->Import(cx, aURI, &global, &exports));

  JS::RootedValue ctor(cx);
  if (!JS_GetProperty(cx, exports, aConstructor, &ctor) ||
      !ctor.isObject()) {
    return NS_ERROR_XPC_JSOBJECT_HAS_NO_FUNCTION_NAMED;
  }

  JS::RootedObject inst(cx);
  if (!JS::Construct(cx, ctor, JS::HandleValueArray::empty(), &inst)) {
    return NS_ERROR_FAILURE;
  }

  return nsContentUtils::XPConnect()->WrapJS(cx, inst, NS_GET_IID(nsISupports),
                                             (void**)aResult);
}


const StaticModule gStaticModules[] = {
  
          /* Anonymous166 */ {
            /* {ce7d7da0-fb28-44a3-8c7b-000c165918f4} */
            { 0xce7d7da0, 0xfb28, 0x44a3, { 0x8c, 0x7b, 0x00, 0x0c, 0x16, 0x59, 0x18, 0xf4 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous157 */ {
            /* {9c7ec5d1-23f9-11d5-aea8-8fcc0793e97f} */
            { 0x9c7ec5d1, 0x23f9, 0x11d5, { 0xae, 0xa8, 0x8f, 0xcc, 0x07, 0x93, 0xe9, 0x7f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous115 */ {
            /* {02bf7a2a-39d8-4a23-a50c-2cbb085ab7a5} */
            { 0x2bf7a2a, 0x39d8, 0x4a23, { 0xa5, 0x0c, 0x2c, 0xbb, 0x08, 0x5a, 0xb7, 0xa5 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous036 */ {
            /* {96cf7855-dfa9-4c6d-8276-f9705b4890f2} */
            { 0x96cf7855, 0xdfa9, 0x4c6d, { 0x82, 0x76, 0xf9, 0x70, 0x5b, 0x48, 0x90, 0xf2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous039 */ {
            /* {51c65f5d-0de5-4edc-9058-60e50cef77f8} */
            { 0x51c65f5d, 0xde5, 0x4edc, { 0x90, 0x58, 0x60, 0xe5, 0x0c, 0xef, 0x77, 0xf8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous046 */ {
            /* {02b9970c-433d-4cc2-923d-f7028ac66073} */
            { 0x2b9970c, 0x433d, 0x4cc2, { 0x92, 0x3d, 0xf7, 0x02, 0x8a, 0xc6, 0x60, 0x73 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous159 */ {
            /* {dc01dbbb-a5bb-4cbb-82bb-085cce06c0bb} */
            { 0xdc01dbbb, 0xa5bb, 0x4cbb, { 0x82, 0xbb, 0x08, 0x5c, 0xce, 0x06, 0xc0, 0xbb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous093 */ {
            /* {92735ff4-6384-4ad6-8508-757010e149ee} */
            { 0x92735ff4, 0x6384, 0x4ad6, { 0x85, 0x08, 0x75, 0x70, 0x10, 0xe1, 0x49, 0xee } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous360 */ {
            /* {77221d5a-1dd2-11b2-8c69-c710f15d2ed5} */
            { 0x77221d5a, 0x1dd2, 0x11b2, { 0x8c, 0x69, 0xc7, 0x10, 0xf1, 0x5d, 0x2e, 0xd5 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous278 */ {
            /* {749e62f4-60ae-4569-a8a2-de78b649660e} */
            { 0x749e62f4, 0x60ae, 0x4569, { 0xa8, 0xa2, 0xde, 0x78, 0xb6, 0x49, 0x66, 0x0e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous108 */ {
            /* {608b7f6f-4b60-40d6-87ed-d933bf53d8c1} */
            { 0x608b7f6f, 0x4b60, 0x40d6, { 0x87, 0xed, 0xd9, 0x33, 0xbf, 0x53, 0xd8, 0xc1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous195 */ {
            /* {296d0900-f8ef-4df0-9c35-db5862abc58d} */
            { 0x296d0900, 0xf8ef, 0x4df0, { 0x9c, 0x35, 0xdb, 0x58, 0x62, 0xab, 0xc5, 0x8d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous221 */ {
            /* {45a5fe2f-c350-4b86-962d-02d5aaaa955a} */
            { 0x45a5fe2f, 0xc350, 0x4b86, { 0x96, 0x2d, 0x02, 0xd5, 0xaa, 0xaa, 0x95, 0x5a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous192 */ {
            /* {e1676f84-e6e5-45d0-a4bf-d9905efc5b2e} */
            { 0xe1676f84, 0xe6e5, 0x45d0, { 0xa4, 0xbf, 0xd9, 0x90, 0x5e, 0xfc, 0x5b, 0x2e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous103 */ {
            /* {86fb70ec-90ff-45ad-a1c1-f77d3c1184e9} */
            { 0x86fb70ec, 0x90ff, 0x45ad, { 0xa1, 0xc1, 0xf7, 0x7d, 0x3c, 0x11, 0x84, 0xe9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous070 */ {
            /* {dd2bbf2f-3399-4389-8f5f-d382afb8b2d6} */
            { 0xdd2bbf2f, 0x3399, 0x4389, { 0x8f, 0x5f, 0xd3, 0x82, 0xaf, 0xb8, 0xb2, 0xd6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous284 */ {
            /* {c47ff942-9678-44a5-bc9b-05e0d676c79c} */
            { 0xc47ff942, 0x9678, 0x44a5, { 0xbc, 0x9b, 0x05, 0xe0, 0xd6, 0x76, 0xc7, 0x9c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous273 */ {
            /* {b43c96be-2b3a-4dc4-90e9-b06d34219b68} */
            { 0xb43c96be, 0x2b3a, 0x4dc4, { 0x90, 0xe9, 0xb0, 0x6d, 0x34, 0x21, 0x9b, 0x68 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous047 */ {
            /* {1775081b-b62d-4954-8ffe-a067bbf508a7} */
            { 0x1775081b, 0xb62d, 0x4954, { 0x8f, 0xfe, 0xa0, 0x67, 0xbb, 0xf5, 0x08, 0xa7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous320 */ {
            /* {9111de73-9322-4bfc-8b65-2b727f3e6ec8} */
            { 0x9111de73, 0x9322, 0x4bfc, { 0x8b, 0x65, 0x2b, 0x72, 0x7f, 0x3e, 0x6e, 0xc8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous289 */ {
            /* {9de95a0c-39a4-4d64-9a53-17940dd7cabb} */
            { 0x9de95a0c, 0x39a4, 0x4d64, { 0x9a, 0x53, 0x17, 0x94, 0x0d, 0xd7, 0xca, 0xbb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous029 */ {
            /* {f30bc0a2-958b-4287-bf62-ce38ba0c811e} */
            { 0xf30bc0a2, 0x958b, 0x4287, { 0xbf, 0x62, 0xce, 0x38, 0xba, 0x0c, 0x81, 0x1e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous315 */ {
            /* {97943eec-0e48-49ef-b7b7-cf4aa0109bb6} */
            { 0x97943eec, 0xe48, 0x49ef, { 0xb7, 0xb7, 0xcf, 0x4a, 0xa0, 0x10, 0x9b, 0xb6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous262 */ {
            /* {92668367-1b17-4190-86b2-1061b2179744} */
            { 0x92668367, 0x1b17, 0x4190, { 0x86, 0xb2, 0x10, 0x61, 0xb2, 0x17, 0x97, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous419 */ {
            /* {bbe50ef2-80eb-469d-b70d-02858275389f} */
            { 0xbbe50ef2, 0x80eb, 0x469d, { 0xb7, 0x0d, 0x02, 0x85, 0x82, 0x75, 0x38, 0x9f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* IO */ {
            /* {9ac9e770-18bc-11d3-9337-00104ba0fd40} */
            { 0x9ac9e770, 0x18bc, 0x11d3, { 0x93, 0x37, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous059 */ {
            /* {0d55a5e6-d185-44f0-b992-a8e1321e4bce} */
            { 0xd55a5e6, 0xd185, 0x44f0, { 0xb9, 0x92, 0xa8, 0xe1, 0x32, 0x1e, 0x4b, 0xce } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous347 */ {
            /* {25db9b8e-8123-4de1-b66d-8bbbedf2cdf4} */
            { 0x25db9b8e, 0x8123, 0x4de1, { 0xb6, 0x6d, 0x8b, 0xbb, 0xed, 0xf2, 0xcd, 0xf4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous074 */ {
            /* {cde1d019-fad8-4044-b141-65fb4fb7a245} */
            { 0xcde1d019, 0xfad8, 0x4044, { 0xb1, 0x41, 0x65, 0xfb, 0x4f, 0xb7, 0xa2, 0x45 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* OfflineCacheUpdate */ {
            /* {ec06f3fc-70db-4ecd-94e0-a6e91ca44d8a} */
            { 0xec06f3fc, 0x70db, 0x4ecd, { 0x94, 0xe0, 0xa6, 0xe9, 0x1c, 0xa4, 0x4d, 0x8a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous408 */ {
            /* {e4a0ee4e-0775-457b-9118-b3ae97a7c758} */
            { 0xe4a0ee4e, 0x775, 0x457b, { 0x91, 0x18, 0xb3, 0xae, 0x97, 0xa7, 0xc7, 0x58 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous048 */ {
            /* {7293e901-2be3-4c02-b4bd-cbef6fc24f78} */
            { 0x7293e901, 0x2be3, 0x4c02, { 0xb4, 0xbd, 0xcb, 0xef, 0x6f, 0xc2, 0x4f, 0x78 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous294 */ {
            /* {d38926e0-29c1-11eb-8588-0800200c9a66} */
            { 0xd38926e0, 0x29c1, 0x11eb, { 0x85, 0x88, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* URILoader */ {
            /* {9f6d5d40-90e7-11d3-af80-00a024ffc08c} */
            { 0x9f6d5d40, 0x90e7, 0x11d3, { 0xaf, 0x80, 0x00, 0xa0, 0x24, 0xff, 0xc0, 0x8c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous172 */ {
            /* {ff41913b-546a-4bff-9201-dc9b2c032eba} */
            { 0xff41913b, 0x546a, 0x4bff, { 0x92, 0x01, 0xdc, 0x9b, 0x2c, 0x03, 0x2e, 0xba } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous438 */ {
            /* {e8ee88b0-5571-4086-a45b-39a716906bdb} */
            { 0xe8ee88b0, 0x5571, 0x4086, { 0xa4, 0x5b, 0x39, 0xa7, 0x16, 0x90, 0x6b, 0xdb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous322 */ {
            /* {ded150e3-c92e-4077-a396-0dba9953e39f} */
            { 0xded150e3, 0xc92e, 0x4077, { 0xa3, 0x96, 0x0d, 0xba, 0x99, 0x53, 0xe3, 0x9f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous006 */ {
            /* {6f8bb968-c14f-4d6f-9733-6c6737b35dce} */
            { 0x6f8bb968, 0xc14f, 0x4d6f, { 0x97, 0x33, 0x6c, 0x67, 0x37, 0xb3, 0x5d, 0xce } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous163 */ {
            /* {fb8cbf4e-4701-4ba1-b1d6-5388e041fb67} */
            { 0xfb8cbf4e, 0x4701, 0x4ba1, { 0xb1, 0xd6, 0x53, 0x88, 0xe0, 0x41, 0xfb, 0x67 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous254 */ {
            /* {cf4c4487-66d9-4e18-a2e9-39002245332f} */
            { 0xcf4c4487, 0x66d9, 0x4e18, { 0xa2, 0xe9, 0x39, 0x00, 0x22, 0x45, 0x33, 0x2f } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous199 */ {
            /* {ee706783-3af8-4d19-9e84-e2ebfe213480} */
            { 0xee706783, 0x3af8, 0x4d19, { 0x9e, 0x84, 0xe2, 0xeb, 0xfe, 0x21, 0x34, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous359 */ {
            /* {8b5314ba-db01-11d2-96ce-0060b0fb9956} */
            { 0x8b5314ba, 0xdb01, 0x11d2, { 0x96, 0xce, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56 } },
            { 0x20c2 } /* "@mozilla.org/widget/clipboard;1" */,
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous052 */ {
            /* {d974b814-8fde-411c-8c45-b86791b81030} */
            { 0xd974b814, 0x8fde, 0x411c, { 0x8c, 0x45, 0xb8, 0x67, 0x91, 0xb8, 0x10, 0x30 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous067 */ {
            /* {184385cb-2d35-4b99-a9a3-7c780bf66b9b} */
            { 0x184385cb, 0x2d35, 0x4b99, { 0xa9, 0xa3, 0x7c, 0x78, 0x0b, 0xf6, 0x6b, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous402 */ {
            /* {c6e47036-ca94-4be3-963a-9abd8705f7a8} */
            { 0xc6e47036, 0xca94, 0x4be3, { 0x96, 0x3a, 0x9a, 0xbd, 0x87, 0x05, 0xf7, 0xa8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous217 */ {
            /* {36a1d3b3-d886-4317-96ff-87b0005cfef7} */
            { 0x36a1d3b3, 0xd886, 0x4317, { 0x96, 0xff, 0x87, 0xb0, 0x00, 0x5c, 0xfe, 0xf7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous014 */ {
            /* {efbd7b87-9b15-4684-abf0-dc2679daadb1} */
            { 0xefbd7b87, 0x9b15, 0x4684, { 0xab, 0xf0, 0xdc, 0x26, 0x79, 0xda, 0xad, 0xb1 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous165 */ {
            /* {04445aa0-fd27-4c99-bd41-6be6318ae92c} */
            { 0x4445aa0, 0xfd27, 0x4c99, { 0xbd, 0x41, 0x6b, 0xe6, 0x31, 0x8a, 0xe9, 0x2c } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous173 */ {
            /* {78804a84-8173-42b6-bb94-789f0816a810} */
            { 0x78804a84, 0x8173, 0x42b6, { 0xbb, 0x94, 0x78, 0x9f, 0x08, 0x16, 0xa8, 0x10 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous131 */ {
            /* {c272fee0-c7e9-11d3-8cda-0060b0fc14a3} */
            { 0xc272fee0, 0xc7e9, 0x11d3, { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous343 */ {
            /* {7e677795-c582-4cd1-9e8d-8271b3474d2a} */
            { 0x7e677795, 0xc582, 0x4cd1, { 0x9e, 0x8d, 0x82, 0x71, 0xb3, 0x47, 0x4d, 0x2a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous445 */ {
            /* {4aef38b7-6364-4e23-a5e7-12f837fbbd9c} */
            { 0x4aef38b7, 0x6364, 0x4e23, { 0xa5, 0xe7, 0x12, 0xf8, 0x37, 0xfb, 0xbd, 0x9c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous118 */ {
            /* {62147d1e-ef6a-40e8-aaf8-d039f5caaa81} */
            { 0x62147d1e, 0xef6a, 0x40e8, { 0xaa, 0xf8, 0xd0, 0x39, 0xf5, 0xca, 0xaa, 0x81 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous319 */ {
            /* {ca168834-cc00-48f9-b83c-fd018e58cae3} */
            { 0xca168834, 0xcc00, 0x48f9, { 0xb8, 0x3c, 0xfd, 0x01, 0x8e, 0x58, 0xca, 0xe3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous183 */ {
            /* {56388dad-287b-4240-a785-85c394012503} */
            { 0x56388dad, 0x287b, 0x4240, { 0xa7, 0x85, 0x85, 0xc3, 0x94, 0x01, 0x25, 0x03 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous337 */ {
            /* {e3a1f3c9-3ae1-4b40-a5e0-7b457fc9a9ad} */
            { 0xe3a1f3c9, 0x3ae1, 0x4b40, { 0xa5, 0xe0, 0x7b, 0x45, 0x7f, 0xc9, 0xa9, 0xad } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous434 */ {
            /* {5a75c25a-5e7e-4d90-8f7c-07eb15cc0aa8} */
            { 0x5a75c25a, 0x5e7e, 0x4d90, { 0x8f, 0x7c, 0x07, 0xeb, 0x15, 0xcc, 0x0a, 0xa8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous325 */ {
            /* {6f46b6f4-c8b1-4bd4-a4fa-9ebbed0753ea} */
            { 0x6f46b6f4, 0xc8b1, 0x4bd4, { 0xa4, 0xfa, 0x9e, 0xbb, 0xed, 0x07, 0x53, 0xea } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous096 */ {
            /* {2bc2ad62-ad5d-4b7b-a9db-f74ae203c527} */
            { 0x2bc2ad62, 0xad5d, 0x4b7b, { 0xa9, 0xdb, 0xf7, 0x4a, 0xe2, 0x03, 0xc5, 0x27 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous042 */ {
            /* {3e43ee93-829e-4ea6-a34e-62d9e4c9f993} */
            { 0x3e43ee93, 0x829e, 0x4ea6, { 0xa3, 0x4e, 0x62, 0xd9, 0xe4, 0xc9, 0xf9, 0x93 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous466 */ {
            /* {69da374a-fda3-4a93-9fbc-d9304f66a7fe} */
            { 0x69da374a, 0xfda3, 0x4a93, { 0x9f, 0xbc, 0xd9, 0x30, 0x4f, 0x66, 0xa7, 0xfe } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous317 */ {
            /* {f376627f-0bbc-47b8-887e-fc92574cc91f} */
            { 0xf376627f, 0xbbc, 0x47b8, { 0x88, 0x7e, 0xfc, 0x92, 0x57, 0x4c, 0xc9, 0x1f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous293 */ {
            /* {705a423f-2f69-42f3-b9fe-1517e0dee56f} */
            { 0x705a423f, 0x2f69, 0x42f3, { 0xb9, 0xfe, 0x15, 0x17, 0xe0, 0xde, 0xe5, 0x6f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* ThirdPartyUtil */ {
            /* {08c6cc8b-cfb0-421d-b1f7-683ff2989681} */
            { 0x8c6cc8b, 0xcfb0, 0x421d, { 0xb1, 0xf7, 0x68, 0x3f, 0xf2, 0x98, 0x96, 0x81 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous239 */ {
            /* {bbbb1d61-438f-4436-92ed-8308e5830fb0} */
            { 0xbbbb1d61, 0x438f, 0x4436, { 0x92, 0xed, 0x83, 0x08, 0xe5, 0x83, 0x0f, 0xb0 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous051 */ {
            /* {4fff5d46-d827-4cd4-a970-8fd53977440e} */
            { 0x4fff5d46, 0xd827, 0x4cd4, { 0xa9, 0x70, 0x8f, 0xd5, 0x39, 0x77, 0x44, 0x0e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous229 */ {
            /* {4ea60761-31d6-491d-9e34-4b53a26c416c} */
            { 0x4ea60761, 0x31d6, 0x491d, { 0x9e, 0x34, 0x4b, 0x53, 0xa2, 0x6c, 0x41, 0x6c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous019 */ {
            /* {3a16d383-92bd-4c24-ac10-0e2bd66883ab} */
            { 0x3a16d383, 0x92bd, 0x4c24, { 0xac, 0x10, 0x0e, 0x2b, 0xd6, 0x68, 0x83, 0xab } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* SocketTransport */ {
            /* {ad56b25f-e6bb-4db3-9f7b-5b7db33fd2b1} */
            { 0xad56b25f, 0xe6bb, 0x4db3, { 0x9f, 0x7b, 0x5b, 0x7d, 0xb3, 0x3f, 0xd2, 0xb1 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous309 */ {
            /* {aa892eb4-ffbf-477d-9f9a-06c995ae9f27} */
            { 0xaa892eb4, 0xffbf, 0x477d, { 0x9f, 0x9a, 0x06, 0xc9, 0x95, 0xae, 0x9f, 0x27 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Directory */ {
            /* {f00152d0-b40b-11d3-8c9c-000064657374} */
            { 0xf00152d0, 0xb40b, 0x11d3, { 0x8c, 0x9c, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous263 */ {
            /* {c887b6a9-a5eb-4566-a440-bebaea3e54fd} */
            { 0xc887b6a9, 0xa5eb, 0x4566, { 0xa4, 0x40, 0xbe, 0xba, 0xea, 0x3e, 0x54, 0xfd } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous298 */ {
            /* {0636a680-45cb-11e4-916c-0800200c9a66} */
            { 0x636a680, 0x45cb, 0x11e4, { 0x91, 0x6c, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous206 */ {
            /* {518e071f-1dd2-11b2-937e-c45f14def778} */
            { 0x518e071f, 0x1dd2, 0x11b2, { 0x93, 0x7e, 0xc4, 0x5f, 0x14, 0xde, 0xf7, 0x78 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous060 */ {
            /* {e031267e-bec8-4f3c-b0b1-396b77ca260c} */
            { 0xe031267e, 0xbec8, 0x4f3c, { 0xb0, 0xb1, 0x39, 0x6b, 0x77, 0xca, 0x26, 0x0c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous233 */ {
            /* {bea13a3a-44e8-4d7f-a0a2-2c67f84e3a97} */
            { 0xbea13a3a, 0x44e8, 0x4d7f, { 0xa0, 0xa2, 0x2c, 0x67, 0xf8, 0x4e, 0x3a, 0x97 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous458 */ {
            /* {e1e79dec-4085-4994-ac5b-744b016697e6} */
            { 0xe1e79dec, 0x4085, 0x4994, { 0xac, 0x5b, 0x74, 0x4b, 0x01, 0x66, 0x97, 0xe6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* UrlClassifierDB */ {
            /* {7a258022-6765-11e5-b379-b37b1f2354be} */
            { 0x7a258022, 0x6765, 0x11e5, { 0xb3, 0x79, 0xb3, 0x7b, 0x1f, 0x23, 0x54, 0xbe } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous149 */ {
            /* {fbc81170-1f69-11d3-9344-00104ba0fd40} */
            { 0xfbc81170, 0x1f69, 0x11d3, { 0x93, 0x44, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous100 */ {
            /* {cdb4757f-f51b-40c0-8b38-66d12c3bff7b} */
            { 0xcdb4757f, 0xf51b, 0x40c0, { 0x8b, 0x38, 0x66, 0xd1, 0x2c, 0x3b, 0xff, 0x7b } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* GfxInfo */ {
            /* {d755a760-9f27-11df-0800-200c9a664242} */
            { 0xd755a760, 0x9f27, 0x11df, { 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66, 0x42, 0x42 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_PROCESS,
          },
  
          /* Anonymous367 */ {
            /* {a6cf9129-15b3-11d2-932e-00805f8add32} */
            { 0xa6cf9129, 0x15b3, 0x11d2, { 0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous350 */ {
            /* {c401eb80-f9ea-11d3-bb6f-e732b73ebe7c} */
            { 0xc401eb80, 0xf9ea, 0x11d3, { 0xbb, 0x6f, 0xe7, 0x32, 0xb7, 0x3e, 0xbe, 0x7c } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_AND_MAIN_PROCESS,
          },
  
          /* Anonymous440 */ {
            /* {7b121f7e-ebe4-43ab-9410-dc9087a1dba6} */
            { 0x7b121f7e, 0xebe4, 0x43ab, { 0x94, 0x10, 0xdc, 0x90, 0x87, 0xa1, 0xdb, 0xa6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous456 */ {
            /* {2a058404-fb85-44ec-8cfd-e8cbdc988dc1} */
            { 0x2a058404, 0xfb85, 0x44ec, { 0x8c, 0xfd, 0xe8, 0xcb, 0xdc, 0x98, 0x8d, 0xc1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous227 */ {
            /* {4fe082ae-6ff0-4b41-b24f-eaa664f6e46a} */
            { 0x4fe082ae, 0x6ff0, 0x4b41, { 0xb2, 0x4f, 0xea, 0xa6, 0x64, 0xf6, 0xe4, 0x6a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous474 */ {
            /* {3b265b69-f813-48ff-880d-d88d101af404} */
            { 0x3b265b69, 0xf813, 0x48ff, { 0x88, 0x0d, 0xd8, 0x8d, 0x10, 0x1a, 0xf4, 0x04 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous225 */ {
            /* {16955eee-6c48-4152-9309-c42a465138a1} */
            { 0x16955eee, 0x6c48, 0x4152, { 0x93, 0x09, 0xc4, 0x2a, 0x46, 0x51, 0x38, 0xa1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous448 */ {
            /* {6030f7ef-32ed-46a7-9a63-6a5d3f90445f} */
            { 0x6030f7ef, 0x32ed, 0x46a7, { 0x9a, 0x63, 0x6a, 0x5d, 0x3f, 0x90, 0x44, 0x5f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous439 */ {
            /* {cf7fd51f-aba2-44c1-9ff0-11f7508efcd4} */
            { 0xcf7fd51f, 0xaba2, 0x44c1, { 0x9f, 0xf0, 0x11, 0xf7, 0x50, 0x8e, 0xfc, 0xd4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* UrlClassifierUtils */ {
            /* {b7b2ccec-7912-4ea6-a548-b038447004bd} */
            { 0xb7b2ccec, 0x7912, 0x4ea6, { 0xa5, 0x48, 0xb0, 0x38, 0x44, 0x70, 0x04, 0xbd } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* HttpActivityDistributor */ {
            /* {15629ada-a41c-4a09-961f-6553cd60b1a2} */
            { 0x15629ada, 0xa41c, 0x4a09, { 0x96, 0x1f, 0x65, 0x53, 0xcd, 0x60, 0xb1, 0xa2 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous174 */ {
            /* {275d800e-3f60-4896-adb7-d7f390ce0e42} */
            { 0x275d800e, 0x3f60, 0x4896, { 0xad, 0xb7, 0xd7, 0xf3, 0x90, 0xce, 0x0e, 0x42 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous084 */ {
            /* {f6fcd651-164b-4416-b001-9c8c393fd93b} */
            { 0xf6fcd651, 0x164b, 0x4416, { 0xb0, 0x01, 0x9c, 0x8c, 0x39, 0x3f, 0xd9, 0x3b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous255 */ {
            /* {580530e5-118c-4bc7-ab88-bc2cd2b97223} */
            { 0x580530e5, 0x118c, 0x4bc7, { 0xab, 0x88, 0xbc, 0x2c, 0xd2, 0xb9, 0x72, 0x23 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous427 */ {
            /* {8d2f40b2-4875-4c95-97d9-3f7dca2cb460} */
            { 0x8d2f40b2, 0x4875, 0x4c95, { 0x97, 0xd9, 0x3f, 0x7d, 0xca, 0x2c, 0xb4, 0x60 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous316 */ {
            /* {b322a5c0-a419-484e-96ba-d7182163899f} */
            { 0xb322a5c0, 0xa419, 0x484e, { 0x96, 0xba, 0xd7, 0x18, 0x21, 0x63, 0x89, 0x9f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous158 */ {
            /* {dc01db59-a513-4c90-824b-085cce06c0aa} */
            { 0xdc01db59, 0xa513, 0x4c90, { 0x82, 0x4b, 0x08, 0x5c, 0xce, 0x06, 0xc0, 0xaa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous175 */ {
            /* {892ffeb0-3f80-11d3-a16c-0050041caf44} */
            { 0x892ffeb0, 0x3f80, 0x11d3, { 0xa1, 0x6c, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous249 */ {
            /* {17a59a6b-92b8-42e5-bce0-ab434c7a7135} */
            { 0x17a59a6b, 0x92b8, 0x42e5, { 0xbc, 0xe0, 0xab, 0x43, 0x4c, 0x7a, 0x71, 0x35 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous076 */ {
            /* {b6f2f870-b0bc-4a1a-9c40-02cc171adb5b} */
            { 0xb6f2f870, 0xb0bc, 0x4a1a, { 0x9c, 0x40, 0x02, 0xcc, 0x17, 0x1a, 0xdb, 0x5b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous420 */ {
            /* {f5475c51-59a7-4757-b3d9-e211a9410872} */
            { 0xf5475c51, 0x59a7, 0x4757, { 0xb3, 0xd9, 0xe2, 0x11, 0xa9, 0x41, 0x08, 0x72 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous423 */ {
            /* {0ddf4df8-4dbb-4133-8b79-9afb966514f5} */
            { 0xddf4df8, 0x4dbb, 0x4133, { 0x8b, 0x79, 0x9a, 0xfb, 0x96, 0x65, 0x14, 0xf5 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous424 */ {
            /* {3b55e72e-ab7e-431b-89c0-3b06a8b14016} */
            { 0x3b55e72e, 0xab7e, 0x431b, { 0x89, 0xc0, 0x3b, 0x06, 0xa8, 0xb1, 0x40, 0x16 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous079 */ {
            /* {ba5bc4c6-1dd1-11b2-bb89-b844c6ec0339} */
            { 0xba5bc4c6, 0x1dd1, 0x11b2, { 0xbb, 0x89, 0xb8, 0x44, 0xc6, 0xec, 0x03, 0x39 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous197 */ {
            /* {14a50f2b-7ff6-48a5-88e3-615fd111f5d3} */
            { 0x14a50f2b, 0x7ff6, 0x48a5, { 0x88, 0xe3, 0x61, 0x5f, 0xd1, 0x11, 0xf5, 0xd3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous069 */ {
            /* {a2b5ef38-a650-4bee-b488-3739f1f3afe7} */
            { 0xa2b5ef38, 0xa650, 0x4bee, { 0xb4, 0x88, 0x37, 0x39, 0xf1, 0xf3, 0xaf, 0xe7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous235 */ {
            /* {5e756573-234a-49ea-bbe4-59ec7a70657d} */
            { 0x5e756573, 0x234a, 0x49ea, { 0xbb, 0xe4, 0x59, 0xec, 0x7a, 0x70, 0x65, 0x7d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous021 */ {
            /* {1060afaf-dc9e-43da-8646-23a2faf48493} */
            { 0x1060afaf, 0xdc9e, 0x43da, { 0x86, 0x46, 0x23, 0xa2, 0xfa, 0xf4, 0x84, 0x93 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous431 */ {
            /* {3160e271-138d-4cc7-9d63-6429f16957c7} */
            { 0x3160e271, 0x138d, 0x4cc7, { 0x9d, 0x63, 0x64, 0x29, 0xf1, 0x69, 0x57, 0xc7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous390 */ {
            /* {acf8dc4b-4a25-11d3-9890-006008962422} */
            { 0xacf8dc4b, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous446 */ {
            /* {e7c2aaf5-c11a-4954-9dbf-e28edec1fd91} */
            { 0xe7c2aaf5, 0xc11a, 0x4954, { 0x9d, 0xbf, 0xe2, 0x8e, 0xde, 0xc1, 0xfd, 0x91 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous128 */ {
            /* {510a86bb-6019-4ed1-bb4f-965cffd23ece} */
            { 0x510a86bb, 0x6019, 0x4ed1, { 0xbb, 0x4f, 0x96, 0x5c, 0xff, 0xd2, 0x3e, 0xce } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous346 */ {
            /* {4e4aae11-8901-46cc-8217-dad7c5415873} */
            { 0x4e4aae11, 0x8901, 0x46cc, { 0x82, 0x17, 0xda, 0xd7, 0xc5, 0x41, 0x58, 0x73 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous011 */ {
            /* {7370a02a-4886-42c3-a4ec-d48c726ec30a} */
            { 0x7370a02a, 0x4886, 0x42c3, { 0xa4, 0xec, 0xd4, 0x8c, 0x72, 0x6e, 0xc3, 0x0a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous247 */ {
            /* {2ee3039b-2de4-43d9-93b0-649beacff39a} */
            { 0x2ee3039b, 0x2de4, 0x43d9, { 0x93, 0xb0, 0x64, 0x9b, 0xea, 0xcf, 0xf3, 0x9a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous271 */ {
            /* {98d0e975-9cad-4ce3-ae2f-f878b8be6307} */
            { 0x98d0e975, 0x9cad, 0x4ce3, { 0xae, 0x2f, 0xf8, 0x78, 0xb8, 0xbe, 0x63, 0x07 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous388 */ {
            /* {a99febba-1dd1-11b2-a943-b02334a6d083} */
            { 0xa99febba, 0x1dd1, 0x11b2, { 0xa9, 0x43, 0xb0, 0x23, 0x34, 0xa6, 0xd0, 0x83 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous381 */ {
            /* {aaf68860-f849-40ee-bb7a-b229bce036a3} */
            { 0xaaf68860, 0xf849, 0x40ee, { 0xbb, 0x7a, 0xb2, 0x29, 0xbc, 0xe0, 0x36, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous429 */ {
            /* {ecba5203-77da-465a-865e-78b7af10d8f7} */
            { 0xecba5203, 0x77da, 0x465a, { 0x86, 0x5e, 0x78, 0xb7, 0xaf, 0x10, 0xd8, 0xf7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous156 */ {
            /* {e64f152a-9f07-11d3-8cda-0060b0fc14a3} */
            { 0xe64f152a, 0x9f07, 0x11d3, { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous361 */ {
            /* {8b5314bb-db01-11d2-96ce-0060b0fb9956} */
            { 0x8b5314bb, 0xdb01, 0x11d2, { 0x96, 0xce, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous335 */ {
            /* {898cdc9b-e43f-422f-9cc4-2f6291b415a3} */
            { 0x898cdc9b, 0xe43f, 0x422f, { 0x9c, 0xc4, 0x2f, 0x62, 0x91, 0xb4, 0x15, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous303 */ {
            /* {1a817186-357a-47cd-8aea-2850d60e959e} */
            { 0x1a817186, 0x357a, 0x47cd, { 0x8a, 0xea, 0x28, 0x50, 0xd6, 0x0e, 0x95, 0x9e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous230 */ {
            /* {5516303d-9007-45a0-94b9-940ef134a6e2} */
            { 0x5516303d, 0x9007, 0x45a0, { 0x94, 0xb9, 0x94, 0x0e, 0xf1, 0x34, 0xa6, 0xe2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous295 */ {
            /* {f964a319-397a-4d21-8be6-5cdd1ee3e3ae} */
            { 0xf964a319, 0x397a, 0x4d21, { 0x8b, 0xe6, 0x5c, 0xdd, 0x1e, 0xe3, 0xe3, 0xae } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous187 */ {
            /* {de9472d0-8034-11d3-9399-00104ba0fd40} */
            { 0xde9472d0, 0x8034, 0x11d3, { 0x93, 0x99, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous058 */ {
            /* {5296f79e-15ea-40c3-8196-19cfa64d328c} */
            { 0x5296f79e, 0x15ea, 0x40c3, { 0x81, 0x96, 0x19, 0xcf, 0xa6, 0x4d, 0x32, 0x8c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous265 */ {
            /* {0c797702-1c60-4051-9dd7-4d7405605642} */
            { 0xc797702, 0x1c60, 0x4051, { 0x9d, 0xd7, 0x4d, 0x74, 0x05, 0x60, 0x56, 0x42 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous003 */ {
            /* {47cd0651-b1be-4a0f-b5c4-10e5a573ef71} */
            { 0x47cd0651, 0xb1be, 0x4a0f, { 0xb5, 0xc4, 0x10, 0xe5, 0xa5, 0x73, 0xef, 0x71 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous224 */ {
            /* {16786594-0296-4471-8096-8f84497ca428} */
            { 0x16786594, 0x296, 0x4471, { 0x80, 0x96, 0x8f, 0x84, 0x49, 0x7c, 0xa4, 0x28 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous362 */ {
            /* {6987230e-0098-4e78-bc5f-1493ee7519fa} */
            { 0x6987230e, 0x98, 0x4e78, { 0xbc, 0x5f, 0x14, 0x93, 0xee, 0x75, 0x19, 0xfa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous198 */ {
            /* {f9346d98-f27a-4e89-b744-493843416480} */
            { 0xf9346d98, 0xf27a, 0x4e89, { 0xb7, 0x44, 0x49, 0x38, 0x43, 0x41, 0x64, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous141 */ {
            /* {2693457e-3ba5-4455-991f-5350946adb12} */
            { 0x2693457e, 0x3ba5, 0x4455, { 0x99, 0x1f, 0x53, 0x50, 0x94, 0x6a, 0xdb, 0x12 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous399 */ {
            /* {d962398a-99e5-49b2-857a-c159049c7f6c} */
            { 0xd962398a, 0x99e5, 0x49b2, { 0x85, 0x7a, 0xc1, 0x59, 0x04, 0x9c, 0x7f, 0x6c } },
            { 0x20e2 } /* "@mozilla.org/system-info;1" */,
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous413 */ {
            /* {0099907d-123c-4853-a46a-43098b5fb68c} */
            { 0x99907d, 0x123c, 0x4853, { 0xa4, 0x6a, 0x43, 0x09, 0x8b, 0x5f, 0xb6, 0x8c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous400 */ {
            /* {7a4204c6-e45a-4c37-8ebb-6709a22c917c} */
            { 0x7a4204c6, 0xe45a, 0x4c37, { 0x8e, 0xbb, 0x67, 0x09, 0xa2, 0x2c, 0x91, 0x7c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous441 */ {
            /* {01e1c3ff-094a-d048-44b4-62d29c7b4f39} */
            { 0x1e1c3ff, 0x94a, 0xd048, { 0x44, 0xb4, 0x62, 0xd2, 0x9c, 0x7b, 0x4f, 0x39 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous208 */ {
            /* {9ef18451-a157-4d17-8132-47afef213689} */
            { 0x9ef18451, 0xa157, 0x4d17, { 0x81, 0x32, 0x47, 0xaf, 0xef, 0x21, 0x36, 0x89 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous380 */ {
            /* {7b4eeb20-d781-11d4-8a83-0010a4e0c9ca} */
            { 0x7b4eeb20, 0xd781, 0x11d4, { 0x8a, 0x83, 0x00, 0x10, 0xa4, 0xe0, 0xc9, 0xca } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous391 */ {
            /* {acf8dc4c-4a25-11d3-9890-006008962422} */
            { 0xacf8dc4c, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous442 */ {
            /* {130b016f-fad7-4526-bc7f-827dabf79265} */
            { 0x130b016f, 0xfad7, 0x4526, { 0xbc, 0x7f, 0x82, 0x7d, 0xab, 0xf7, 0x92, 0x65 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous234 */ {
            /* {1b7db999-2ecd-4abf-bb95-a726896798ca} */
            { 0x1b7db999, 0x2ecd, 0x4abf, { 0xbb, 0x95, 0xa7, 0x26, 0x89, 0x67, 0x98, 0xca } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous160 */ {
            /* {a181af0d-68b8-4308-94db-d4f859058215} */
            { 0xa181af0d, 0x68b8, 0x4308, { 0x94, 0xdb, 0xd4, 0xf8, 0x59, 0x05, 0x82, 0x15 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* FindService */ {
            /* {5060b803-340e-11d5-be5b-b3e063ec6a3c} */
            { 0x5060b803, 0x340e, 0x11d5, { 0xbe, 0x5b, 0xb3, 0xe0, 0x63, 0xec, 0x6a, 0x3c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous087 */ {
            /* {13a5320c-4c91-4fa4-bd16-b081a3ba8c0b} */
            { 0x13a5320c, 0x4c91, 0x4fa4, { 0xbd, 0x16, 0xb0, 0x81, 0xa3, 0xba, 0x8c, 0x0b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous308 */ {
            /* {7319788a-fe93-4db3-9f39-818cf08f4256} */
            { 0x7319788a, 0xfe93, 0x4db3, { 0x9f, 0x39, 0x81, 0x8c, 0xf0, 0x8f, 0x42, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous430 */ {
            /* {38bd0634-0fd4-46f0-b85f-13ced889eeec} */
            { 0x38bd0634, 0xfd4, 0x46f0, { 0xb8, 0x5f, 0x13, 0xce, 0xd8, 0x89, 0xee, 0xec } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous216 */ {
            /* {1dbc6eb6-0972-4bdb-9dc4-acd0abf72369} */
            { 0x1dbc6eb6, 0x972, 0x4bdb, { 0x9d, 0xc4, 0xac, 0xd0, 0xab, 0xf7, 0x23, 0x69 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous184 */ {
            /* {9c4e9d49-ce64-4ca3-acef-3075c5e5aba7} */
            { 0x9c4e9d49, 0xce64, 0x4ca3, { 0xac, 0xef, 0x30, 0x75, 0xc5, 0xe5, 0xab, 0xa7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous032 */ {
            /* {c616fcfd-9737-41f1-aa74-cee72a38f91b} */
            { 0xc616fcfd, 0x9737, 0x41f1, { 0xaa, 0x74, 0xce, 0xe7, 0x2a, 0x38, 0xf9, 0x1b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous193 */ {
            /* {4ce234f1-52e8-47a9-8c8d-b02f815733c7} */
            { 0x4ce234f1, 0x52e8, 0x47a9, { 0x8c, 0x8d, 0xb0, 0x2f, 0x81, 0x57, 0x33, 0xc7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous007 */ {
            /* {4cec1de4-1671-4fc3-a53e-6c539dc77a26} */
            { 0x4cec1de4, 0x1671, 0x4fc3, { 0xa5, 0x3e, 0x6c, 0x53, 0x9d, 0xc7, 0x7a, 0x26 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous196 */ {
            /* {75a500a2-0030-40f7-86f8-63f225b940ae} */
            { 0x75a500a2, 0x30, 0x40f7, { 0x86, 0xf8, 0x63, 0xf2, 0x25, 0xb9, 0x40, 0xae } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous130 */ {
            /* {be9a53ae-c7e9-11d3-8cda-0060b0fc14a3} */
            { 0xbe9a53ae, 0xc7e9, 0x11d3, { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous267 */ {
            /* {1b4c85df-cbdd-4bb6-b04e-613caece083c} */
            { 0x1b4c85df, 0xcbdd, 0x4bb6, { 0xb0, 0x4e, 0x61, 0x3c, 0xae, 0xce, 0x08, 0x3c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous138 */ {
            /* {e1c61582-2a84-11d3-8cce-0060b0fc14a3} */
            { 0xe1c61582, 0x2a84, 0x11d3, { 0x8c, 0xce, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous016 */ {
            /* {4148c488-dca1-49fc-a621-2a0097a62422} */
            { 0x4148c488, 0xdca1, 0x49fc, { 0xa6, 0x21, 0x2a, 0x00, 0x97, 0xa6, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous150 */ {
            /* {25029490-f132-11d2-9588-00805f369f95} */
            { 0x25029490, 0xf132, 0x11d2, { 0x95, 0x88, 0x00, 0x80, 0x5f, 0x36, 0x9f, 0x95 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous035 */ {
            /* {9f171ac4-0939-4ef8-b360-3408aedc3060} */
            { 0x9f171ac4, 0x939, 0x4ef8, { 0xb3, 0x60, 0x34, 0x08, 0xae, 0xdc, 0x30, 0x60 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous080 */ {
            /* {56c778e4-1bee-45f3-a689-886692a97fe7} */
            { 0x56c778e4, 0x1bee, 0x45f3, { 0xa6, 0x89, 0x88, 0x66, 0x92, 0xa9, 0x7f, 0xe7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous382 */ {
            /* {43ebf210-8a7b-4ddb-a83d-b87c51a058db} */
            { 0x43ebf210, 0x8a7b, 0x4ddb, { 0xa8, 0x3d, 0xb8, 0x7c, 0x51, 0xa0, 0x58, 0xdb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous387 */ {
            /* {cbf86870-4ac0-11d3-baea-00805f8a5dd7} */
            { 0xcbf86870, 0x4ac0, 0x11d3, { 0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous472 */ {
            /* {0ff5ce56-5b09-4db8-adc6-8266af95f864} */
            { 0xff5ce56, 0x5b09, 0x4db8, { 0xad, 0xc6, 0x82, 0x66, 0xaf, 0x95, 0xf8, 0x64 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous463 */ {
            /* {f68037ec-2790-44c5-8e5f-df5da58b93a7} */
            { 0xf68037ec, 0x2790, 0x44c5, { 0x8e, 0x5f, 0xdf, 0x5d, 0xa5, 0x8b, 0x93, 0xa7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous432 */ {
            /* {656db07c-aa80-49e4-bce8-e431baae697d} */
            { 0x656db07c, 0xaa80, 0x49e4, { 0xbc, 0xe8, 0xe4, 0x31, 0xba, 0xae, 0x69, 0x7d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous389 */ {
            /* {acf8dc43-4a25-11d3-9890-006008962422} */
            { 0xacf8dc43, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous342 */ {
            /* {471f4944-1dd2-11b2-87ac-90be0a51d609} */
            { 0x471f4944, 0x1dd2, 0x11b2, { 0x87, 0xac, 0x90, 0xbe, 0x0a, 0x51, 0xd6, 0x09 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous453 */ {
            /* {bfc310d2-38a0-11d3-8cd3-0060b0fc14a3} */
            { 0xbfc310d2, 0x38a0, 0x11d3, { 0x8c, 0xd3, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous145 */ {
            /* {9158c470-86e4-11d4-9be2-00e09872a416} */
            { 0x9158c470, 0x86e4, 0x11d4, { 0x9b, 0xe2, 0x00, 0xe0, 0x98, 0x72, 0xa4, 0x16 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous407 */ {
            /* {565e3a2c-1dd2-11b2-8da1-b4cef17e568d} */
            { 0x565e3a2c, 0x1dd2, 0x11b2, { 0x8d, 0xa1, 0xb4, 0xce, 0xf1, 0x7e, 0x56, 0x8d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous005 */ {
            /* {d8903bf6-68d5-4e97-bcd1-e4d3012f721a} */
            { 0xd8903bf6, 0x68d5, 0x4e97, { 0xbc, 0xd1, 0xe4, 0xd3, 0x01, 0x2f, 0x72, 0x1a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous135 */ {
            /* {5d6352a3-b9c3-4fa3-87aa-b2a3c6e5a501} */
            { 0x5d6352a3, 0xb9c3, 0x4fa3, { 0x87, 0xaa, 0xb2, 0xa3, 0xc6, 0xe5, 0xa5, 0x01 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous212 */ {
            /* {b084a2ce-1dd1-11b2-bf10-8324f8e065cc} */
            { 0xb084a2ce, 0x1dd1, 0x11b2, { 0xbf, 0x10, 0x83, 0x24, 0xf8, 0xe0, 0x65, 0xcc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous300 */ {
            /* {6e134924-6c3a-4d86-81ac-69432dd971dc} */
            { 0x6e134924, 0x6c3a, 0x4d86, { 0x81, 0xac, 0x69, 0x43, 0x2d, 0xd9, 0x71, 0xdc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous218 */ {
            /* {a496d0a2-dff7-4e23-bd65-1ca742fa178a} */
            { 0xa496d0a2, 0xdff7, 0x4e23, { 0xbd, 0x65, 0x1c, 0xa7, 0x42, 0xfa, 0x17, 0x8a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous105 */ {
            /* {c7e410d4-85f2-11d3-9f63-006008a6efe9} */
            { 0xc7e410d4, 0x85f2, 0x11d3, { 0x9f, 0x63, 0x00, 0x60, 0x08, 0xa6, 0xef, 0xe9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous313 */ {
            /* {aea477f2-b3a2-469c-aa29-0a82d132b829} */
            { 0xaea477f2, 0xb3a2, 0x469c, { 0xaa, 0x29, 0x0a, 0x82, 0xd1, 0x32, 0xb8, 0x29 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous245 */ {
            /* {42906796-d16a-44a1-b518-0f108ab38eba} */
            { 0x42906796, 0xd16a, 0x44a1, { 0xb5, 0x18, 0x0f, 0x10, 0x8a, 0xb3, 0x8e, 0xba } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous241 */ {
            /* {3c9c43b6-09eb-4ed2-9b87-e29f4221eef0} */
            { 0x3c9c43b6, 0x9eb, 0x4ed2, { 0x9b, 0x87, 0xe2, 0x9f, 0x42, 0x21, 0xee, 0xf0 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous272 */ {
            /* {3fdd6c28-5b87-4e3e-8b57-8e83c23c1a6d} */
            { 0x3fdd6c28, 0x5b87, 0x4e3e, { 0x8b, 0x57, 0x8e, 0x83, 0xc2, 0x3c, 0x1a, 0x6d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous290 */ {
            /* {88cecbb7-6c63-4b3b-8cd4-84f3b8228c69} */
            { 0x88cecbb7, 0x6c63, 0x4b3b, { 0x8c, 0xd4, 0x84, 0xf3, 0xb8, 0x22, 0x8c, 0x69 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous114 */ {
            /* {62d4b190-3642-4450-b019-d1c1fba56025} */
            { 0x62d4b190, 0x3642, 0x4450, { 0xb0, 0x19, 0xd1, 0xc1, 0xfb, 0xa5, 0x60, 0x25 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* DBusHandlerApp */ {
            /* {6c3c274b-4cbf-4bb5-a635-05ad2cbb6535} */
            { 0x6c3c274b, 0x4cbf, 0x4bb5, { 0xa6, 0x35, 0x05, 0xad, 0x2c, 0xbb, 0x65, 0x35 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous164 */ {
            /* {2be14592-28d4-4a83-8fe9-08e778849f6e} */
            { 0x2be14592, 0x28d4, 0x4a83, { 0x8f, 0xe9, 0x08, 0xe7, 0x78, 0x84, 0x9f, 0x6e } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous179 */ {
            /* {7584ce90-5b25-11d3-a175-0050041caf44} */
            { 0x7584ce90, 0x5b25, 0x11d3, { 0xa1, 0x75, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous378 */ {
            /* {00bd71fb-7f09-4ec3-96af-a0b522b77969} */
            { 0xbd71fb, 0x7f09, 0x4ec3, { 0x96, 0xaf, 0xa0, 0xb5, 0x22, 0xb7, 0x79, 0x69 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous375 */ {
            /* {678c50b8-6bcb-4ad0-b9b8-c81175955199} */
            { 0x678c50b8, 0x6bcb, 0x4ad0, { 0xb9, 0xb8, 0xc8, 0x11, 0x75, 0x95, 0x51, 0x99 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous062 */ {
            /* {8c72bcdb-0c37-4786-a9e5-510afa2f8ede} */
            { 0x8c72bcdb, 0xc37, 0x4786, { 0xa9, 0xe5, 0x51, 0x0a, 0xfa, 0x2f, 0x8e, 0xde } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* PermissionManager */ {
            /* {4f6b5e00-0c36-11d5-a535-0010a401eb10} */
            { 0x4f6b5e00, 0xc36, 0x11d5, { 0xa5, 0x35, 0x00, 0x10, 0xa4, 0x01, 0xeb, 0x10 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous425 */ {
            /* {cdcc1ab8-3cea-4e6c-a294-a651fa35227f} */
            { 0xcdcc1ab8, 0x3cea, 0x4e6c, { 0xa2, 0x94, 0xa6, 0x51, 0xfa, 0x35, 0x22, 0x7f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous081 */ {
            /* {7ef52eaf-b7e1-462b-87e2-5d1dbaca9048} */
            { 0x7ef52eaf, 0xb7e1, 0x462b, { 0x87, 0xe2, 0x5d, 0x1d, 0xba, 0xca, 0x90, 0x48 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous452 */ {
            /* {4bbe1b96-8956-457f-a03f-9c27435f2afa} */
            { 0x4bbe1b96, 0x8956, 0x457f, { 0xa0, 0x3f, 0x9c, 0x27, 0x43, 0x5f, 0x2a, 0xfa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous190 */ {
            /* {e0da1d70-2f7b-11d3-8cd0-0060b0fc14a3} */
            { 0xe0da1d70, 0x2f7b, 0x11d3, { 0x8c, 0xd0, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous246 */ {
            /* {f6d5ebbd-34f4-487d-9d10-3d34123e3eb9} */
            { 0xf6d5ebbd, 0x34f4, 0x487d, { 0x9d, 0x10, 0x3d, 0x34, 0x12, 0x3e, 0x3e, 0xb9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous372 */ {
            /* {86c37b9a-74e7-4672-844e-6e7dd83ba484} */
            { 0x86c37b9a, 0x74e7, 0x4672, { 0x84, 0x4e, 0x6e, 0x7d, 0xd8, 0x3b, 0xa4, 0x84 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous371 */ {
            /* {c521a612-2aad-46db-b6ab-3b821fb150b1} */
            { 0xc521a612, 0x2aad, 0x46db, { 0xb6, 0xab, 0x3b, 0x82, 0x1f, 0xb1, 0x50, 0xb1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous134 */ {
            /* {a62af1ba-79b3-4896-8aaf-b148bfce4280} */
            { 0xa62af1ba, 0x79b3, 0x4896, { 0x8a, 0xaf, 0xb1, 0x48, 0xbf, 0xce, 0x42, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous222 */ {
            /* {67ba681d-5485-4fff-952c-2ee337ffdcd6} */
            { 0x67ba681d, 0x5485, 0x4fff, { 0x95, 0x2c, 0x2e, 0xe3, 0x37, 0xff, 0xdc, 0xd6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous038 */ {
            /* {b43964aa-a078-44b2-b06b-fd4d1b172e66} */
            { 0xb43964aa, 0xa078, 0x44b2, { 0xb0, 0x6b, 0xfd, 0x4d, 0x1b, 0x17, 0x2e, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous001 */ {
            /* {63c7b9f4-0cc8-43f8-b666-0a661655cb73} */
            { 0x63c7b9f4, 0xcc8, 0x43f8, { 0xb6, 0x66, 0x0a, 0x66, 0x16, 0x55, 0xcb, 0x73 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous220 */ {
            /* {2a35dd47-b026-4e8d-b6b7-5740f61ab902} */
            { 0x2a35dd47, 0xb026, 0x4e8d, { 0xb6, 0xb7, 0x57, 0x40, 0xf6, 0x1a, 0xb9, 0x02 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous215 */ {
            /* {fb0bbc5c-452e-4783-b32c-80124693d871} */
            { 0xfb0bbc5c, 0x452e, 0x4783, { 0xb3, 0x2c, 0x80, 0x12, 0x46, 0x93, 0xd8, 0x71 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous082 */ {
            /* {a6cf9115-15b3-11d2-932e-00805f8add32} */
            { 0xa6cf9115, 0x15b3, 0x11d2, { 0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous330 */ {
            /* {7beb3ba8-6ec3-41b4-b67c-da89b8518922} */
            { 0x7beb3ba8, 0x6ec3, 0x41b4, { 0xb6, 0x7c, 0xda, 0x89, 0xb8, 0x51, 0x89, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous012 */ {
            /* {dfcd2adc-7867-4d3a-ba70-17501f208142} */
            { 0xdfcd2adc, 0x7867, 0x4d3a, { 0xba, 0x70, 0x17, 0x50, 0x1f, 0x20, 0x81, 0x42 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous066 */ {
            /* {cccd665f-edf3-41fc-ab9b-fc55b37340aa} */
            { 0xcccd665f, 0xedf3, 0x41fc, { 0xab, 0x9b, 0xfc, 0x55, 0xb3, 0x73, 0x40, 0xaa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous143 */ {
            /* {e9b301c0-e0e4-11d3-a1a8-0050041caf44} */
            { 0xe9b301c0, 0xe0e4, 0x11d3, { 0xa1, 0xa8, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous405 */ {
            /* {35c66fd1-95e9-4e0a-80c5-c3bd2b375481} */
            { 0x35c66fd1, 0x95e9, 0x4e0a, { 0x80, 0xc5, 0xc3, 0xbd, 0x2b, 0x37, 0x54, 0x81 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous306 */ {
            /* {bf1e01d0-953e-11df-981c-0800200c9a66} */
            { 0xbf1e01d0, 0x953e, 0x11df, { 0x98, 0x1c, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous170 */ {
            /* {1813cbb4-c98e-4622-8c7d-839167f3f272} */
            { 0x1813cbb4, 0xc98e, 0x4622, { 0x8c, 0x7d, 0x83, 0x91, 0x67, 0xf3, 0xf2, 0x72 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous403 */ {
            /* {16d222a6-1dd2-11b2-b693-f38b02c021b2} */
            { 0x16d222a6, 0x1dd2, 0x11b2, { 0xb6, 0x93, 0xf3, 0x8b, 0x02, 0xc0, 0x21, 0xb2 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous203 */ {
            /* {2ce606b0-bee6-11d1-aad9-00805f8a3e14} */
            { 0x2ce606b0, 0xbee6, 0x11d1, { 0xaa, 0xd9, 0x00, 0x80, 0x5f, 0x8a, 0x3e, 0x14 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous089 */ {
            /* {ac2bb8fe-eeeb-4572-b40f-be03932b56e0} */
            { 0xac2bb8fe, 0xeeeb, 0x4572, { 0xb4, 0x0f, 0xbe, 0x03, 0x93, 0x2b, 0x56, 0xe0 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous015 */ {
            /* {97bf9550-2a7b-11e9-b56e-0800200c9a66} */
            { 0x97bf9550, 0x2a7b, 0x11e9, { 0xb5, 0x6e, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous443 */ {
            /* {bfcb82c2-5611-4318-90d6-baf4a7864252} */
            { 0xbfcb82c2, 0x5611, 0x4318, { 0x90, 0xd6, 0xba, 0xf4, 0xa7, 0x86, 0x42, 0x52 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous307 */ {
            /* {3a0012eb-007f-4bb8-aa81-a07385f77a25} */
            { 0x3a0012eb, 0x7f, 0x4bb8, { 0xaa, 0x81, 0xa0, 0x73, 0x85, 0xf7, 0x7a, 0x25 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous127 */ {
            /* {4ffae79e-57bd-4d7a-a0c9-0045a17b3615} */
            { 0x4ffae79e, 0x57bd, 0x4d7a, { 0xa0, 0xc9, 0x00, 0x45, 0xa1, 0x7b, 0x36, 0x15 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous204 */ {
            /* {8f685a9d-8181-46d6-a71d-869289099c6d} */
            { 0x8f685a9d, 0x8181, 0x46d6, { 0xa7, 0x1d, 0x86, 0x92, 0x89, 0x09, 0x9c, 0x6d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous162 */ {
            /* {2ec62893-3b35-48fa-ab1d-5e68a9f45f08} */
            { 0x2ec62893, 0x3b35, 0x48fa, { 0xab, 0x1d, 0x5e, 0x68, 0xa9, 0xf4, 0x5f, 0x08 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous461 */ {
            /* {653e0e4d-3ee4-45fa-b272-97c20bc01eb8} */
            { 0x653e0e4d, 0x3ee4, 0x45fa, { 0xb2, 0x72, 0x97, 0xc2, 0x0b, 0xc0, 0x1e, 0xb8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous404 */ {
            /* {91775d60-d5dc-11d2-92fb-00e09805570f} */
            { 0x91775d60, 0xd5dc, 0x11d2, { 0x92, 0xfb, 0x00, 0xe0, 0x98, 0x05, 0x57, 0x0f } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous055 */ {
            /* {e7d52d9e-c148-47d8-ab2a-95d7f40ea53d} */
            { 0xe7d52d9e, 0xc148, 0x47d8, { 0xab, 0x2a, 0x95, 0xd7, 0xf4, 0x0e, 0xa5, 0x3d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous194 */ {
            /* {3ff8fb9f-ee63-48df-89f0-dace0242fd82} */
            { 0x3ff8fb9f, 0xee63, 0x48df, { 0x89, 0xf0, 0xda, 0xce, 0x02, 0x42, 0xfd, 0x82 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous465 */ {
            /* {7ee2a4c0-4b93-17d3-ba18-0060b0f199a2} */
            { 0x7ee2a4c0, 0x4b93, 0x17d3, { 0xba, 0x18, 0x00, 0x60, 0xb0, 0xf1, 0x99, 0xa2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous110 */ {
            /* {430d416c-a722-4ad1-be98-d9a445f85e3f} */
            { 0x430d416c, 0xa722, 0x4ad1, { 0xbe, 0x98, 0xd9, 0xa4, 0x45, 0xf8, 0x5e, 0x3f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous075 */ {
            /* {daaa8d73-677e-4233-8acd-2c404bd01658} */
            { 0xdaaa8d73, 0x677e, 0x4233, { 0x8a, 0xcd, 0x2c, 0x40, 0x4b, 0xd0, 0x16, 0x58 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous094 */ {
            /* {65944815-e9ae-48bd-a2bf-f1108720950c} */
            { 0x65944815, 0xe9ae, 0x48bd, { 0xa2, 0xbf, 0xf1, 0x10, 0x87, 0x20, 0x95, 0x0c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous283 */ {
            /* {c00c432d-a0c9-46d7-bef6-9c45b4d07341} */
            { 0xc00c432d, 0xa0c9, 0x46d7, { 0xbe, 0xf6, 0x9c, 0x45, 0xb4, 0xd0, 0x73, 0x41 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous349 */ {
            /* {2d96b3df-c051-11d1-a827-0040959a28c9} */
            { 0x2d96b3df, 0xc051, 0x11d1, { 0xa8, 0x27, 0x00, 0x40, 0x95, 0x9a, 0x28, 0xc9 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous040 */ {
            /* {b93af7a1-3411-44a8-bd0a-8af3dde4d8d8} */
            { 0xb93af7a1, 0x3411, 0x44a8, { 0xbd, 0x0a, 0x8a, 0xf3, 0xdd, 0xe4, 0xd8, 0xd8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous147 */ {
            /* {9e3b6c90-2f75-11d3-8cd0-0060b0fc14a3} */
            { 0x9e3b6c90, 0x2f75, 0x11d3, { 0x8c, 0xd0, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous219 */ {
            /* {9d383ddd-6856-4187-8485-f36195b29a0e} */
            { 0x9d383ddd, 0x6856, 0x4187, { 0x84, 0x85, 0xf3, 0x61, 0x95, 0xb2, 0x9a, 0x0e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous459 */ {
            /* {1d9bb10c-c0ab-4fe8-9e4f-4058b8519832} */
            { 0x1d9bb10c, 0xc0ab, 0x4fe8, { 0x9e, 0x4f, 0x40, 0x58, 0xb8, 0x51, 0x98, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous044 */ {
            /* {d1748d4c-7f6a-4dc5-add6-d55b7678537e} */
            { 0xd1748d4c, 0x7f6a, 0x4dc5, { 0xad, 0xd6, 0xd5, 0x5b, 0x76, 0x78, 0x53, 0x7e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous034 */ {
            /* {e740ddb4-18b4-4aac-8ae1-9b0f4320769d} */
            { 0xe740ddb4, 0x18b4, 0x4aac, { 0x8a, 0xe1, 0x9b, 0x0f, 0x43, 0x20, 0x76, 0x9d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous228 */ {
            /* {16e5c837-f877-4e23-9c64-eddf905e30e6} */
            { 0x16e5c837, 0xf877, 0x4e23, { 0x9c, 0x64, 0xed, 0xdf, 0x90, 0x5e, 0x30, 0xe6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous451 */ {
            /* {041a129f-10ce-4bda-a60d-e027a26d5ed0} */
            { 0x41a129f, 0x10ce, 0x4bda, { 0xa6, 0x0d, 0xe0, 0x27, 0xa2, 0x6d, 0x5e, 0xd0 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous287 */ {
            /* {e8b8bdb7-c96c-4d82-9c6f-2b3c585ec7ea} */
            { 0xe8b8bdb7, 0xc96c, 0x4d82, { 0x9c, 0x6f, 0x2b, 0x3c, 0x58, 0x5e, 0xc7, 0xea } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous333 */ {
            /* {b3c290a6-3943-4b89-8bbe-c01eb7b3b311} */
            { 0xb3c290a6, 0x3943, 0x4b89, { 0x8b, 0xbe, 0xc0, 0x1e, 0xb7, 0xb3, 0xb3, 0x11 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous411 */ {
            /* {03d68f92-9513-4e25-9be9-7cb239874172} */
            { 0x3d68f92, 0x9513, 0x4e25, { 0x9b, 0xe9, 0x7c, 0xb2, 0x39, 0x87, 0x41, 0x72 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous435 */ {
            /* {ae2793c0-2ba3-4adb-9c5e-c23525812c64} */
            { 0xae2793c0, 0x2ba3, 0x4adb, { 0x9c, 0x5e, 0xc2, 0x35, 0x25, 0x81, 0x2c, 0x64 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous117 */ {
            /* {6eae857e-4ba9-11e3-9b39-b4036188709b} */
            { 0x6eae857e, 0x4ba9, 0x11e3, { 0x9b, 0x39, 0xb4, 0x03, 0x61, 0x88, 0x70, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous207 */ {
            /* {4cb64dfd-ca98-4e24-befd-0d9285a33bcb} */
            { 0x4cb64dfd, 0xca98, 0x4e24, { 0xbe, 0xfd, 0x0d, 0x92, 0x85, 0xa3, 0x3b, 0xcb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous177 */ {
            /* {7d7008a0-c49a-11d3-9b22-0080c7cb1080} */
            { 0x7d7008a0, 0xc49a, 0x11d3, { 0x9b, 0x22, 0x00, 0x80, 0xc7, 0xcb, 0x10, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous393 */ {
            /* {acf8dc49-4a25-11d3-9890-006008962422} */
            { 0xacf8dc49, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous213 */ {
            /* {ff9fbcd7-9517-4334-b97a-ceed78909974} */
            { 0xff9fbcd7, 0x9517, 0x4334, { 0xb9, 0x7a, 0xce, 0xed, 0x78, 0x90, 0x99, 0x74 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous107 */ {
            /* {19d9161b-a2a9-4518-b2c9-fcb8296d6dcd} */
            { 0x19d9161b, 0xa2a9, 0x4518, { 0xb2, 0xc9, 0xfc, 0xb8, 0x29, 0x6d, 0x6d, 0xcd } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous365 */ {
            /* {2f977d53-5485-11d4-87e2-0010a4e75ef2} */
            { 0x2f977d53, 0x5485, 0x11d4, { 0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* LoginReputation */ {
            /* {91fa9e67-1427-4ee9-8ee0-1a6ed578bee1} */
            { 0x91fa9e67, 0x1427, 0x4ee9, { 0x8e, 0xe0, 0x1a, 0x6e, 0xd5, 0x78, 0xbe, 0xe1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous469 */ {
            /* {31689828-da66-49a6-870c-df62b83fe789} */
            { 0x31689828, 0xda66, 0x49a6, { 0x87, 0x0c, 0xdf, 0x62, 0xb8, 0x3f, 0xe7, 0x89 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* DocLoader */ {
            /* {057b04d0-0ccf-11d2-beba-00805f8a66dc} */
            { 0x57b04d0, 0xccf, 0x11d2, { 0xbe, 0xba, 0x00, 0x80, 0x5f, 0x8a, 0x66, 0xdc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* ChromeRegistry */ {
            /* {47049e42-1d87-482a-984d-56ae185e367a} */
            { 0x47049e42, 0x1d87, 0x482a, { 0x98, 0x4d, 0x56, 0xae, 0x18, 0x5e, 0x36, 0x7a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous111 */ {
            /* {91ca2441-050f-4f7c-9df8-75b40ea40156} */
            { 0x91ca2441, 0x50f, 0x4f7c, { 0x9d, 0xf8, 0x75, 0xb4, 0x0e, 0xa4, 0x01, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous068 */ {
            /* {23e8fd98-a625-4b08-be1a-f7cc18a5b106} */
            { 0x23e8fd98, 0xa625, 0x4b08, { 0xbe, 0x1a, 0xf7, 0xcc, 0x18, 0xa5, 0xb1, 0x06 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous395 */ {
            /* {acf8dc47-4a25-11d3-9890-006008962422} */
            { 0xacf8dc47, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous356 */ {
            /* {fc2389b8-c650-4093-9e42-b05e5f0685b7} */
            { 0xfc2389b8, 0xc650, 0x4093, { 0x9e, 0x42, 0xb0, 0x5e, 0x5f, 0x06, 0x85, 0xb7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous026 */ {
            /* {95790842-75a0-430d-98bf-f5ce3788ea6d} */
            { 0x95790842, 0x75a0, 0x430d, { 0x98, 0xbf, 0xf5, 0xce, 0x37, 0x88, 0xea, 0x6d } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous467 */ {
            /* {4a6212db-accb-11d3-b765-0060b0b6cecb} */
            { 0x4a6212db, 0xaccb, 0x11d3, { 0xb7, 0x65, 0x00, 0x60, 0xb0, 0xb6, 0xce, 0xcb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous154 */ {
            /* {450a2b55-620a-44b3-9f67-839b3b0c329c} */
            { 0x450a2b55, 0x620a, 0x44b3, { 0x9f, 0x67, 0x83, 0x9b, 0x3b, 0x0c, 0x32, 0x9c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous396 */ {
            /* {acf8dc48-4a25-11d3-9890-006008962422} */
            { 0xacf8dc48, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous244 */ {
            /* {ab94809d-33f0-4f28-af38-01efbd3baf22} */
            { 0xab94809d, 0x33f0, 0x4f28, { 0xaf, 0x38, 0x01, 0xef, 0xbd, 0x3b, 0xaf, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous120 */ {
            /* {a2027ec6-ba0d-4c72-805d-148233f5f33c} */
            { 0xa2027ec6, 0xba0d, 0x4c72, { 0x80, 0x5d, 0x14, 0x82, 0x33, 0xf5, 0xf3, 0x3c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* ApplicationReputation */ {
            /* {d21b4c33-716f-4117-8041-2770b59ff8a6} */
            { 0xd21b4c33, 0x716f, 0x4117, { 0x80, 0x41, 0x27, 0x70, 0xb5, 0x9f, 0xf8, 0xa6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous090 */ {
            /* {38d1592e-b81e-432b-86f8-471878bbfe07} */
            { 0x38d1592e, 0xb81e, 0x432b, { 0x86, 0xf8, 0x47, 0x18, 0x78, 0xbb, 0xfe, 0x07 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous237 */ {
            /* {d28f8a0b-95da-48f4-b712-caf37097be41} */
            { 0xd28f8a0b, 0x95da, 0x48f4, { 0xb7, 0x12, 0xca, 0xf3, 0x70, 0x97, 0xbe, 0x41 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous468 */ {
            /* {caaab47f-1e31-478e-8919-970904e9cb72} */
            { 0xcaaab47f, 0x1e31, 0x478e, { 0x89, 0x19, 0x97, 0x09, 0x04, 0xe9, 0xcb, 0x72 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous171 */ {
            /* {c9f74572-7b8e-4fec-bb4a-03c0d3021bd6} */
            { 0xc9f74572, 0x7b8e, 0x4fec, { 0xbb, 0x4a, 0x03, 0xc0, 0xd3, 0x02, 0x1b, 0xd6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous043 */ {
            /* {bdc2e533-b308-4708-ac8e-a8bfade6d851} */
            { 0xbdc2e533, 0xb308, 0x4708, { 0xac, 0x8e, 0xa8, 0xbf, 0xad, 0xe6, 0xd8, 0x51 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous357 */ {
            /* {8b5314bc-db01-11d2-96ce-0060b0fb9956} */
            { 0x8b5314bc, 0xdb01, 0x11d2, { 0x96, 0xce, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous027 */ {
            /* {c4b6fb7c-bfb1-49dc-a65f-035796524b53} */
            { 0xc4b6fb7c, 0xbfb1, 0x49dc, { 0xa6, 0x5f, 0x03, 0x57, 0x96, 0x52, 0x4b, 0x53 } },
            { 0xffffffff },
            Module::ProcessSelector::CONTENT_PROCESS_ONLY,
          },
  
          /* Anonymous057 */ {
            /* {37f819b0-0b5c-11e3-8ffd-0800200c9a66} */
            { 0x37f819b0, 0xb5c, 0x11e3, { 0x8f, 0xfd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* AlertNotification */ {
            /* {9a7b7a41-0b47-47f7-b61b-15a210d6f020} */
            { 0x9a7b7a41, 0xb47, 0x47f7, { 0xb6, 0x1b, 0x15, 0xa2, 0x10, 0xd6, 0xf0, 0x20 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* StringBundle */ {
            /* {d85a17c1-aa7c-11d2-9b8c-00805f8a16d9} */
            { 0xd85a17c1, 0xaa7c, 0x11d2, { 0x9b, 0x8c, 0x00, 0x80, 0x5f, 0x8a, 0x16, 0xd9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous181 */ {
            /* {77c0e42a-1dd2-11b2-8ebf-edc6606f2f4b} */
            { 0x77c0e42a, 0x1dd2, 0x11b2, { 0x8e, 0xbf, 0xed, 0xc6, 0x60, 0x6f, 0x2f, 0x4b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous056 */ {
            /* {acf6e493-0092-4b26-b172-241e375c57ab} */
            { 0xacf6e493, 0x92, 0x4b26, { 0xb1, 0x72, 0x24, 0x1e, 0x37, 0x5c, 0x57, 0xab } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous398 */ {
            /* {acf8dc42-4a25-11d3-9890-006008962422} */
            { 0xacf8dc42, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous191 */ {
            /* {c375fa80-150f-11d6-a618-0010a401eb10} */
            { 0xc375fa80, 0x150f, 0x11d6, { 0xa6, 0x18, 0x00, 0x10, 0xa4, 0x01, 0xeb, 0x10 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous258 */ {
            /* {c8d0b0b3-17f8-458b-9264-7b67b288fe79} */
            { 0xc8d0b0b3, 0x17f8, 0x458b, { 0x92, 0x64, 0x7b, 0x67, 0xb2, 0x88, 0xfe, 0x79 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous428 */ {
            /* {1147d32c-215b-4014-b180-07fe7aedf915} */
            { 0x1147d32c, 0x215b, 0x4014, { 0xb1, 0x80, 0x07, 0xfe, 0x7a, 0xed, 0xf9, 0x15 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous045 */ {
            /* {3610c242-654e-11e6-8ec0-6d1be389a607} */
            { 0x3610c242, 0x654e, 0x11e6, { 0x8e, 0xc0, 0x6d, 0x1b, 0xe3, 0x89, 0xa6, 0x07 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous176 */ {
            /* {cf0f71fd-fafd-4e2b-9fdc-134d972e16e2} */
            { 0xcf0f71fd, 0xfafd, 0x4e2b, { 0x9f, 0xdc, 0x13, 0x4d, 0x97, 0x2e, 0x16, 0xe2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous236 */ {
            /* {74b89fb0-f200-4ae8-a3ec-dd164117f6de} */
            { 0x74b89fb0, 0xf200, 0x4ae8, { 0xa3, 0xec, 0xdd, 0x16, 0x41, 0x17, 0xf6, 0xde } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous275 */ {
            /* {47a45e5f-691e-4799-8686-14f8d3fc0f8c} */
            { 0x47a45e5f, 0x691e, 0x4799, { 0x86, 0x86, 0x14, 0xf8, 0xd3, 0xfc, 0x0f, 0x8c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous415 */ {
            /* {1691e1f4-ee41-11d4-9885-00c04fa0cf4b} */
            { 0x1691e1f4, 0xee41, 0x11d4, { 0x98, 0x85, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous327 */ {
            /* {66354bc9-7ed1-4692-ae1d-8da97d6b205e} */
            { 0x66354bc9, 0x7ed1, 0x4692, { 0xae, 0x1d, 0x8d, 0xa9, 0x7d, 0x6b, 0x20, 0x5e } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous447 */ {
            /* {9d3f70da-86e9-11d4-95ec-00b0d03e37b7} */
            { 0x9d3f70da, 0x86e9, 0x11d4, { 0x95, 0xec, 0x00, 0xb0, 0xd0, 0x3e, 0x37, 0xb7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous291 */ {
            /* {bbc23860-2553-479d-8b78-94d9038334f7} */
            { 0xbbc23860, 0x2553, 0x479d, { 0x8b, 0x78, 0x94, 0xd9, 0x03, 0x83, 0x34, 0xf7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous363 */ {
            /* {d3f69889-e13a-4321-980c-a39332e21f34} */
            { 0xd3f69889, 0xe13a, 0x4321, { 0x98, 0x0c, 0xa3, 0x93, 0x32, 0xe2, 0x1f, 0x34 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous077 */ {
            /* {77da64d3-7458-4920-9491-86cc9914f904} */
            { 0x77da64d3, 0x7458, 0x4920, { 0x94, 0x91, 0x86, 0xcc, 0x99, 0x14, 0xf9, 0x04 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous240 */ {
            /* {3b667ee0-d2da-4ccc-9c3d-95f2ca6a8b4c} */
            { 0x3b667ee0, 0xd2da, 0x4ccc, { 0x9c, 0x3d, 0x95, 0xf2, 0xca, 0x6a, 0x8b, 0x4c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous182 */ {
            /* {2f277c00-0eaf-4ddb-b936-41326ba48aae} */
            { 0x2f277c00, 0xeaf, 0x4ddb, { 0xb9, 0x36, 0x41, 0x32, 0x6b, 0xa4, 0x8a, 0xae } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous243 */ {
            /* {90d1fd17-2018-4e16-b73c-a04a26fa6dd4} */
            { 0x90d1fd17, 0x2018, 0x4e16, { 0xb7, 0x3c, 0xa0, 0x4a, 0x26, 0xfa, 0x6d, 0xd4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous392 */ {
            /* {acf8dc4d-4a25-11d3-9890-006008962422} */
            { 0xacf8dc4d, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous188 */ {
            /* {dea9657c-18cf-4984-bde9-ccef5d8ab473} */
            { 0xdea9657c, 0x18cf, 0x4984, { 0xbd, 0xe9, 0xcc, 0xef, 0x5d, 0x8a, 0xb4, 0x73 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous086 */ {
            /* {3d8fa16d-c9e1-4b50-bdef-2c7ae249967a} */
            { 0x3d8fa16d, 0xc9e1, 0x4b50, { 0xbd, 0xef, 0x2c, 0x7a, 0xe2, 0x49, 0x96, 0x7a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous231 */ {
            /* {5118a6f9-2493-4f97-9552-620663e03cb3} */
            { 0x5118a6f9, 0x2493, 0x4f97, { 0x95, 0x52, 0x62, 0x06, 0x63, 0xe0, 0x3c, 0xb3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous020 */ {
            /* {9e9a9283-0ce9-4e4a-8f1c-ba129a032c32} */
            { 0x9e9a9283, 0xce9, 0x4e4a, { 0x8f, 0x1c, 0xba, 0x12, 0x9a, 0x03, 0x2c, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous010 */ {
            /* {47f75963-840b-4950-a1f0-d9c1864f8b8e} */
            { 0x47f75963, 0x840b, 0x4950, { 0xa1, 0xf0, 0xd9, 0xc1, 0x86, 0x4f, 0x8b, 0x8e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous202 */ {
            /* {b9f4fd03-9d87-4bfd-9958-85a821750ddc} */
            { 0xb9f4fd03, 0x9d87, 0x4bfd, { 0x99, 0x58, 0x85, 0xa8, 0x21, 0x75, 0x0d, 0xdc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous119 */ {
            /* {208de7fc-a781-4031-bbae-cc0de539f61a} */
            { 0x208de7fc, 0xa781, 0x4031, { 0xbb, 0xae, 0xcc, 0x0d, 0xe5, 0x39, 0xf6, 0x1a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous358 */ {
            /* {e221df9b-3d66-4045-9a66-5720949f8d10} */
            { 0xe221df9b, 0x3d66, 0x4045, { 0x9a, 0x66, 0x57, 0x20, 0x94, 0x9f, 0x8d, 0x10 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous139 */ {
            /* {1f4dbcf7-245c-4c8c-943d-8a1da0495e8a} */
            { 0x1f4dbcf7, 0x245c, 0x4c8c, { 0x94, 0x3d, 0x8a, 0x1d, 0xa0, 0x49, 0x5e, 0x8a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous065 */ {
            /* {62c01e69-9ca4-4060-99e4-b95f628c8e6d} */
            { 0x62c01e69, 0x9ca4, 0x4060, { 0x99, 0xe4, 0xb9, 0x5f, 0x62, 0x8c, 0x8e, 0x6d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* ExtensionPolicy */ {
            /* {562de129-8338-482c-bb96-a1ff09ee53cc} */
            { 0x562de129, 0x8338, 0x482c, { 0xbb, 0x96, 0xa1, 0xff, 0x09, 0xee, 0x53, 0xcc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous326 */ {
            /* {f68578eb-6ec2-4169-ae19-8c6243f0abe1} */
            { 0xf68578eb, 0x6ec2, 0x4169, { 0xae, 0x19, 0x8c, 0x62, 0x43, 0xf0, 0xab, 0xe1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous433 */ {
            /* {64e4bf03-773e-408e-939a-e11652fdfd28} */
            { 0x64e4bf03, 0x773e, 0x408e, { 0x93, 0x9a, 0xe1, 0x16, 0x52, 0xfd, 0xfd, 0x28 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous232 */ {
            /* {d2716568-f5fa-4989-91dd-e11599e932a1} */
            { 0xd2716568, 0xf5fa, 0x4989, { 0x91, 0xdd, 0xe1, 0x15, 0x99, 0xe9, 0x32, 0xa1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous374 */ {
            /* {7e3ff85c-1dd2-11b2-8d4b-eb452cb0ff40} */
            { 0x7e3ff85c, 0x1dd2, 0x11b2, { 0x8d, 0x4b, 0xeb, 0x45, 0x2c, 0xb0, 0xff, 0x40 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous449 */ {
            /* {abc622ea-9655-4123-80d9-22621bdd5465} */
            { 0xabc622ea, 0x9655, 0x4123, { 0x80, 0xd9, 0x22, 0x62, 0x1b, 0xdd, 0x54, 0x65 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous437 */ {
            /* {baa34652-f1f1-4185-b224-244ee82a413a} */
            { 0xbaa34652, 0xf1f1, 0x4185, { 0xb2, 0x24, 0x24, 0x4e, 0xe8, 0x2a, 0x41, 0x3a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous248 */ {
            /* {f66bc334-1dd1-11b2-bab2-90e04fe15c19} */
            { 0xf66bc334, 0x1dd1, 0x11b2, { 0xba, 0xb2, 0x90, 0xe0, 0x4f, 0xe1, 0x5c, 0x19 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous189 */ {
            /* {50d50ddf-f16a-4652-8705-936b19c3763b} */
            { 0x50d50ddf, 0xf16a, 0x4652, { 0x87, 0x05, 0x93, 0x6b, 0x19, 0xc3, 0x76, 0x3b } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous256 */ {
            /* {2e59cc70-f83a-412f-89d4-453885837217} */
            { 0x2e59cc70, 0xf83a, 0x412f, { 0x89, 0xd4, 0x45, 0x38, 0x85, 0x83, 0x72, 0x17 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous352 */ {
            /* {0f872c8c-3ee6-46bd-92a2-69652c6b474e} */
            { 0xf872c8c, 0x3ee6, 0x46bd, { 0x92, 0xa2, 0x69, 0x65, 0x2c, 0x6b, 0x47, 0x4e } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous410 */ {
            /* {0abb0835-5000-4790-af28-61b3ba17c295} */
            { 0xabb0835, 0x5000, 0x4790, { 0xaf, 0x28, 0x61, 0xb3, 0xba, 0x17, 0xc2, 0x95 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous053 */ {
            /* {74b2122d-65a8-4824-aa9e-3d664cb75dc2} */
            { 0x74b2122d, 0x65a8, 0x4824, { 0xaa, 0x9e, 0x3d, 0x66, 0x4c, 0xb7, 0x5d, 0xc2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous344 */ {
            /* {a21bfa01-f349-4394-a84c-8de5cf0737d0} */
            { 0xa21bfa01, 0xf349, 0x4394, { 0xa8, 0x4c, 0x8d, 0xe5, 0xcf, 0x07, 0x37, 0xd0 } },
            { 0x20fd } /* "@mozilla.org/embedcomp/window-watcher;1" */,
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous276 */ {
            /* {6ab96943-a163-482c-9622-4faedc0e827f} */
            { 0x6ab96943, 0xa163, 0x482c, { 0x96, 0x22, 0x4f, 0xae, 0xdc, 0x0e, 0x82, 0x7f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous186 */ {
            /* {b3cfeb91-332a-46c9-ad97-93ff39841494} */
            { 0xb3cfeb91, 0x332a, 0x46c9, { 0xad, 0x97, 0x93, 0xff, 0x39, 0x84, 0x14, 0x94 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous018 */ {
            /* {ec5aa99c-7abb-4142-ac5f-aab2419e38e2} */
            { 0xec5aa99c, 0x7abb, 0x4142, { 0xac, 0x5f, 0xaa, 0xb2, 0x41, 0x9e, 0x38, 0xe2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous140 */ {
            /* {58a1c31c-1dd2-11b2-a3f6-d36949d48268} */
            { 0x58a1c31c, 0x1dd2, 0x11b2, { 0xa3, 0xf6, 0xd3, 0x69, 0x49, 0xd4, 0x82, 0x68 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous286 */ {
            /* {d0c5195d-e798-49d4-b1d3-9324328b2291} */
            { 0xd0c5195d, 0xe798, 0x49d4, { 0xb1, 0xd3, 0x93, 0x24, 0x32, 0x8b, 0x22, 0x91 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous180 */ {
            /* {14c0e880-623e-11d3-a178-0050041caf44} */
            { 0x14c0e880, 0x623e, 0x11d3, { 0xa1, 0x78, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous125 */ {
            /* {c79eb3c6-091a-45a6-8544-5a8d1ab79537} */
            { 0xc79eb3c6, 0x91a, 0x45a6, { 0x85, 0x44, 0x5a, 0x8d, 0x1a, 0xb7, 0x95, 0x37 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous274 */ {
            /* {35ec195a-e8d0-4300-83af-c8a2cc84b4a3} */
            { 0x35ec195a, 0xe8d0, 0x4300, { 0x83, 0xaf, 0xc8, 0xa2, 0xcc, 0x84, 0xb4, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous462 */ {
            /* {2fc2d3e3-020f-404e-b06a-6ecf3ea2334a} */
            { 0x2fc2d3e3, 0x20f, 0x404e, { 0xb0, 0x6a, 0x6e, 0xcf, 0x3e, 0xa2, 0x33, 0x4a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous205 */ {
            /* {0d1bb02e-ac91-4904-b61d-97da83ebf6fb} */
            { 0xd1bb02e, 0xac91, 0x4904, { 0xb6, 0x1d, 0x97, 0xda, 0x83, 0xeb, 0xf6, 0xfb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous269 */ {
            /* {f1e424f2-67fe-4f69-a8f8-3993a71f44fa} */
            { 0xf1e424f2, 0x67fe, 0x4f69, { 0xa8, 0xf8, 0x39, 0x93, 0xa7, 0x1f, 0x44, 0xfa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* AppStartup */ {
            /* {7dd4d320-c84b-4624-8d45-7bb9b2356977} */
            { 0x7dd4d320, 0xc84b, 0x4624, { 0x8d, 0x45, 0x7b, 0xb9, 0xb2, 0x35, 0x69, 0x77 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* AddonContentPolicy */ {
            /* {c26a8241-ecf4-4aed-9f3c-f1f5c713b9a5} */
            { 0xc26a8241, 0xecf4, 0x4aed, { 0x9f, 0x3c, 0xf1, 0xf5, 0xc7, 0x13, 0xb9, 0xa5 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Prefetch */ {
            /* {6b8bdffc-3394-417d-be83-a81b7c0f63bf} */
            { 0x6b8bdffc, 0x3394, 0x417d, { 0xbe, 0x83, 0xa8, 0x1b, 0x7c, 0x0f, 0x63, 0xbf } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous379 */ {
            /* {fb97e4f5-32dd-497a-baa2-7d1e55079910} */
            { 0xfb97e4f5, 0x32dd, 0x497a, { 0xba, 0xa2, 0x7d, 0x1e, 0x55, 0x07, 0x99, 0x10 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous460 */ {
            /* {c9d023f4-6228-4c07-8b1d-9c19573faa27} */
            { 0xc9d023f4, 0x6228, 0x4c07, { 0x8b, 0x1d, 0x9c, 0x19, 0x57, 0x3f, 0xaa, 0x27 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous161 */ {
            /* {d6ef593d-a429-4b14-a887-d9e2f765d9ed} */
            { 0xd6ef593d, 0xa429, 0x4b14, { 0xa8, 0x87, 0xd9, 0xe2, 0xf7, 0x65, 0xd9, 0xed } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous279 */ {
            /* {2bdac17c-53f1-4896-a521-682ccdeef3a8} */
            { 0x2bdac17c, 0x53f1, 0x4896, { 0xa5, 0x21, 0x68, 0x2c, 0xcd, 0xee, 0xf3, 0xa8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous277 */ {
            /* {cb9e0de8-3598-4ed7-857b-827f011ad5d8} */
            { 0xcb9e0de8, 0x3598, 0x4ed7, { 0x85, 0x7b, 0x82, 0x7f, 0x01, 0x1a, 0xd5, 0xd8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous310 */ {
            /* {22117140-9c6e-11d3-aaf1-00805f8a4905} */
            { 0x22117140, 0x9c6e, 0x11d3, { 0xaa, 0xf1, 0x00, 0x80, 0x5f, 0x8a, 0x49, 0x05 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous223 */ {
            /* {be65e2b7-fe46-4e0f-88e0-4b385db4d68a} */
            { 0xbe65e2b7, 0xfe46, 0x4e0f, { 0x88, 0xe0, 0x4b, 0x38, 0x5d, 0xb4, 0xd6, 0x8a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous401 */ {
            /* {0d6ea1d0-879c-11d5-90ef-0010a4e73d9a} */
            { 0xd6ea1d0, 0x879c, 0x11d5, { 0x90, 0xef, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous369 */ {
            /* {30a04e40-38e7-11d4-8cf5-0060b0fc14a3} */
            { 0x30a04e40, 0x38e7, 0x11d4, { 0x8c, 0xf5, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous050 */ {
            /* {0fb47c47-a205-4583-a9fc-cbadf8c95880} */
            { 0xfb47c47, 0xa205, 0x4583, { 0xa9, 0xfc, 0xcb, 0xad, 0xf8, 0xc9, 0x58, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous144 */ {
            /* {3decd6c8-30ef-11d3-8cd0-0060b0fc14a3} */
            { 0x3decd6c8, 0x30ef, 0x11d3, { 0x8c, 0xd0, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous386 */ {
            /* {cbf86871-4ac0-11d3-baea-00805f8a5dd7} */
            { 0xcbf86871, 0x4ac0, 0x11d3, { 0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* NativeFileWatcher */ {
            /* {6f488507-469d-4350-a68d-99c807be0a78} */
            { 0x6f488507, 0x469d, 0x4350, { 0xa6, 0x8d, 0x99, 0xc8, 0x07, 0xbe, 0x0a, 0x78 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* ServiceWorkerManager */ {
            /* {c74bde32-bcc7-4840-8430-c733351b212a} */
            { 0xc74bde32, 0xbcc7, 0x4840, { 0x84, 0x30, 0xc7, 0x33, 0x35, 0x1b, 0x21, 0x2a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous351 */ {
            /* {a9339876-0027-430f-b953-84c9c11c2da3} */
            { 0xa9339876, 0x27, 0x430f, { 0xb9, 0x53, 0x84, 0xc9, 0xc1, 0x1c, 0x2d, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous355 */ {
            /* {b148eed2-236d-11d3-b35c-00a0cc3c1cde} */
            { 0xb148eed2, 0x236d, 0x11d3, { 0xb3, 0x5c, 0x00, 0xa0, 0xcc, 0x3c, 0x1c, 0xde } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous151 */ {
            /* {4f47e42e-4d23-4dd3-bfda-eb29255e9ea3} */
            { 0x4f47e42e, 0x4d23, 0x4dd3, { 0xbf, 0xda, 0xeb, 0x29, 0x25, 0x5e, 0x9e, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous113 */ {
            /* {3014dde6-aa1c-41db-87d0-48764a3710f6} */
            { 0x3014dde6, 0xaa1c, 0x41db, { 0x87, 0xd0, 0x48, 0x76, 0x4a, 0x37, 0x10, 0xf6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous455 */ {
            /* {bd066e5f-146f-4472-8331-7bfd05b1ed90} */
            { 0xbd066e5f, 0x146f, 0x4472, { 0x83, 0x31, 0x7b, 0xfd, 0x05, 0xb1, 0xed, 0x90 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous009 */ {
            /* {91185366-ba97-4438-acba-48deaca63386} */
            { 0x91185366, 0xba97, 0x4438, { 0xac, 0xba, 0x48, 0xde, 0xac, 0xa6, 0x33, 0x86 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous281 */ {
            /* {0f2f347c-1e4f-40cc-8efd-792dea70a85e} */
            { 0xf2f347c, 0x1e4f, 0x40cc, { 0x8e, 0xfd, 0x79, 0x2d, 0xea, 0x70, 0xa8, 0x5e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous426 */ {
            /* {09d9ed1a-e5d4-4004-bfe0-27ceb923d9ac} */
            { 0x9d9ed1a, 0xe5d4, 0x4004, { 0xbf, 0xe0, 0x27, 0xce, 0xb9, 0x23, 0xd9, 0xac } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous312 */ {
            /* {efc1415c-5708-41cc-8226-82bf1d3bee16} */
            { 0xefc1415c, 0x5708, 0x41cc, { 0x82, 0x26, 0x82, 0xbf, 0x1d, 0x3b, 0xee, 0x16 } },
            { 0xffffffff },
            Module::ProcessSelector::CONTENT_PROCESS_ONLY,
          },
  
          /* Anonymous280 */ {
            /* {8aa66d77-1bbb-45a6-991e-b8f47751c291} */
            { 0x8aa66d77, 0x1bbb, 0x45a6, { 0x99, 0x1e, 0xb8, 0xf4, 0x77, 0x51, 0xc2, 0x91 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous097 */ {
            /* {ff8780a5-bbb1-4bc5-8ee7-057e7bc5c925} */
            { 0xff8780a5, 0xbbb1, 0x4bc5, { 0x8e, 0xe7, 0x05, 0x7e, 0x7b, 0xc5, 0xc9, 0x25 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous146 */ {
            /* {7fa5237d-b0eb-438f-9e50-ca0166e63788} */
            { 0x7fa5237d, 0xb0eb, 0x438f, { 0x9e, 0x50, 0xca, 0x01, 0x66, 0xe6, 0x37, 0x88 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous259 */ {
            /* {0c06583d-7dd8-4293-b1a5-912205f779aa} */
            { 0xc06583d, 0x7dd8, 0x4293, { 0xb1, 0xa5, 0x91, 0x22, 0x05, 0xf7, 0x79, 0xaa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous214 */ {
            /* {660a3226-915c-4ffb-bb20-8985a632df05} */
            { 0x660a3226, 0x915c, 0x4ffb, { 0xbb, 0x20, 0x89, 0x85, 0xa6, 0x32, 0xdf, 0x05 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous072 */ {
            /* {f4079b8b-ede5-4b90-a112-5b415a931deb} */
            { 0xf4079b8b, 0xede5, 0x4b90, { 0xa1, 0x12, 0x5b, 0x41, 0x5a, 0x93, 0x1d, 0xeb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous373 */ {
            /* {61ba33c0-3031-11d3-8cd0-0060b0fc14a3} */
            { 0x61ba33c0, 0x3031, 0x11d3, { 0x8c, 0xd0, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous253 */ {
            /* {f3dcf644-79e8-4f59-a1bb-878454488ef9} */
            { 0xf3dcf644, 0x79e8, 0x4f59, { 0xa1, 0xbb, 0x87, 0x84, 0x54, 0x48, 0x8e, 0xf9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous288 */ {
            /* {984e3259-9266-49cf-b605-60b022a00756} */
            { 0x984e3259, 0x9266, 0x49cf, { 0xb6, 0x05, 0x60, 0xb0, 0x22, 0xa0, 0x07, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous397 */ {
            /* {acf8dc44-4a25-11d3-9890-006008962422} */
            { 0xacf8dc44, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous121 */ {
            /* {9226888e-da08-11d3-8cda-0060b0fc14a3} */
            { 0x9226888e, 0xda08, 0x11d3, { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous109 */ {
            /* {461cd5dd-73c6-47a4-8cc3-603b37d84a61} */
            { 0x461cd5dd, 0x73c6, 0x47a4, { 0x8c, 0xc3, 0x60, 0x3b, 0x37, 0xd8, 0x4a, 0x61 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous155 */ {
            /* {1423e739-782c-4081-b5d8-fe6fba68c0ef} */
            { 0x1423e739, 0x782c, 0x4081, { 0xb5, 0xd8, 0xfe, 0x6f, 0xba, 0x68, 0xc0, 0xef } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous464 */ {
            /* {1950539a-90f0-4d22-b5af-71329c68fa35} */
            { 0x1950539a, 0x90f0, 0x4d22, { 0xb5, 0xaf, 0x71, 0x32, 0x9c, 0x68, 0xfa, 0x35 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous095 */ {
            /* {a1b72850-a999-11d2-9119-006008a6edf6} */
            { 0xa1b72850, 0xa999, 0x11d2, { 0x91, 0x19, 0x00, 0x60, 0x08, 0xa6, 0xed, 0xf6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous064 */ {
            /* {52fc3f9f-c0cb-4874-b3d4-ee4b6e9cbe9c} */
            { 0x52fc3f9f, 0xc0cb, 0x4874, { 0xb3, 0xd4, 0xee, 0x4b, 0x6e, 0x9c, 0xbe, 0x9c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous092 */ {
            /* {1460df3b-774c-4205-8349-838e507c3ef9} */
            { 0x1460df3b, 0x774c, 0x4205, { 0x83, 0x49, 0x83, 0x8e, 0x50, 0x7c, 0x3e, 0xf9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous104 */ {
            /* {88e2fd0b-f7f4-480c-9483-7846b00e8dad} */
            { 0x88e2fd0b, 0xf7f4, 0x480c, { 0x94, 0x83, 0x78, 0x46, 0xb0, 0x0e, 0x8d, 0xad } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous444 */ {
            /* {0e3afd3d-eb60-4c2b-963b-56d7c439f124} */
            { 0xe3afd3d, 0xeb60, 0x4c2b, { 0x96, 0x3b, 0x56, 0xd7, 0xc4, 0x39, 0xf1, 0x24 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous004 */ {
            /* {eab9012e-5f74-4cbc-b2b5-a590235513cc} */
            { 0xeab9012e, 0x5f74, 0x4cbc, { 0xb2, 0xb5, 0xa5, 0x90, 0x23, 0x55, 0x13, 0xcc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous013 */ {
            /* {7913837c-9623-11ea-bb37-0242ac130002} */
            { 0x7913837c, 0x9623, 0x11ea, { 0xbb, 0x37, 0x02, 0x42, 0xac, 0x13, 0x00, 0x02 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous311 */ {
            /* {117b219f-92fe-4bd2-a21b-95a342a9d474} */
            { 0x117b219f, 0x92fe, 0x4bd2, { 0xa2, 0x1b, 0x95, 0xa3, 0x42, 0xa9, 0xd4, 0x74 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* UrlClassifierStreamUpdater */ {
            /* {e1797597-f4d6-4dd3-a1e1-745ad352cd80} */
            { 0xe1797597, 0xf4d6, 0x4dd3, { 0xa1, 0xe1, 0x74, 0x5a, 0xd3, 0x52, 0xcd, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* UUIDGenerator */ {
            /* {706d36bb-bf79-4293-81f2-8f6828c18f9d} */
            { 0x706d36bb, 0xbf79, 0x4293, { 0x81, 0xf2, 0x8f, 0x68, 0x28, 0xc1, 0x8f, 0x9d } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous340 */ {
            /* {0fa3158c-d5a7-43de-9181-a285e74cf1d4} */
            { 0xfa3158c, 0xd5a7, 0x43de, { 0x91, 0x81, 0xa2, 0x85, 0xe7, 0x4c, 0xf1, 0xd4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous122 */ {
            /* {9868b4ce-da08-11d3-8cda-0060b0fc14a3} */
            { 0x9868b4ce, 0xda08, 0x11d3, { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous257 */ {
            /* {d9cd00ba-aa4d-47b1-8792-b1fe0cd35060} */
            { 0xd9cd00ba, 0xaa4d, 0x47b1, { 0x87, 0x92, 0xb1, 0xfe, 0x0c, 0xd3, 0x50, 0x60 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous384 */ {
            /* {acf8dc41-4a25-11d3-9890-006008962422} */
            { 0xacf8dc41, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous061 */ {
            /* {b986773e-2b30-4ed2-b8fe-6a96631c8000} */
            { 0xb986773e, 0x2b30, 0x4ed2, { 0xb8, 0xfe, 0x6a, 0x96, 0x63, 0x1c, 0x80, 0x00 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous123 */ {
            /* {6c84aec9-29a5-4264-8fbc-bee8f922ea67} */
            { 0x6c84aec9, 0x29a5, 0x4264, { 0x8f, 0xbc, 0xbe, 0xe8, 0xf9, 0x22, 0xea, 0x67 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous063 */ {
            /* {49a02241-7e48-477a-9345-9f246925dcb3} */
            { 0x49a02241, 0x7e48, 0x477a, { 0x93, 0x45, 0x9f, 0x24, 0x69, 0x25, 0xdc, 0xb3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous377 */ {
            /* {2e23e220-60be-11d3-8c4a-000064657374} */
            { 0x2e23e220, 0x60be, 0x11d3, { 0x8c, 0x4a, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous261 */ {
            /* {e3f772f3-023f-4b32-b074-36cf0fd5d414} */
            { 0xe3f772f3, 0x23f, 0x4b32, { 0xb0, 0x74, 0x36, 0xcf, 0x0f, 0xd5, 0xd4, 0x14 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous025 */ {
            /* {bd6390c8-fbea-11d4-98f6-001083010e9b} */
            { 0xbd6390c8, 0xfbea, 0x11d4, { 0x98, 0xf6, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous054 */ {
            /* {8817b1cf-5ada-43bf-bd73-607657703d0d} */
            { 0x8817b1cf, 0x5ada, 0x43bf, { 0xbd, 0x73, 0x60, 0x76, 0x57, 0x70, 0x3d, 0x0d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous101 */ {
            /* {0365cbd5-d46e-4e94-a39f-83b63cd1a963} */
            { 0x365cbd5, 0xd46e, 0x4e94, { 0xa3, 0x9f, 0x83, 0xb6, 0x3c, 0xd1, 0xa9, 0x63 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous450 */ {
            /* {daf1461b-bf29-4f88-8d0e-4bcdf332c862} */
            { 0xdaf1461b, 0xbf29, 0x4f88, { 0x8d, 0x0e, 0x4b, 0xcd, 0xf3, 0x32, 0xc8, 0x62 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous041 */ {
            /* {9fea635a-2fc2-4d08-9721-d238d3f52f92} */
            { 0x9fea635a, 0x2fc2, 0x4d08, { 0x97, 0x21, 0xd2, 0x38, 0xd3, 0xf5, 0x2f, 0x92 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous250 */ {
            /* {15686f9d-483e-4361-98cd-37f1e8f1e61d} */
            { 0x15686f9d, 0x483e, 0x4361, { 0x98, 0xcd, 0x37, 0xf1, 0xe8, 0xf1, 0xe6, 0x1d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous370 */ {
            /* {67b3ac0c-d806-4d48-939e-6a819e6c248f} */
            { 0x67b3ac0c, 0xd806, 0x4d48, { 0x93, 0x9e, 0x6a, 0x81, 0x9e, 0x6c, 0x24, 0x8f } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous102 */ {
            /* {929814d6-1dd2-11b2-8e08-82fa0a339b00} */
            { 0x929814d6, 0x1dd2, 0x11b2, { 0x8e, 0x08, 0x82, 0xfa, 0x0a, 0x33, 0x9b, 0x00 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous331 */ {
            /* {8866d8e3-4ea5-48b7-a891-13ba0ac15235} */
            { 0x8866d8e3, 0x4ea5, 0x48b7, { 0xa8, 0x91, 0x13, 0xba, 0x0a, 0xc1, 0x52, 0x35 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous028 */ {
            /* {bc0017e3-2438-47be-a567-41db58f17627} */
            { 0xbc0017e3, 0x2438, 0x47be, { 0xa5, 0x67, 0x41, 0xdb, 0x58, 0xf1, 0x76, 0x27 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous454 */ {
            /* {ac9e3e82-bfbd-4f26-941e-f58c8ee178c1} */
            { 0xac9e3e82, 0xbfbd, 0x4f26, { 0x94, 0x1e, 0xf5, 0x8c, 0x8e, 0xe1, 0x78, 0xc1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous116 */ {
            /* {e746a8b1-c97a-4fc5-baa4-66607521bd08} */
            { 0xe746a8b1, 0xc97a, 0x4fc5, { 0xba, 0xa4, 0x66, 0x60, 0x75, 0x21, 0xbd, 0x08 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous137 */ {
            /* {ccd0e960-7947-4635-b70e-4c661b63d675} */
            { 0xccd0e960, 0x7947, 0x4635, { 0xb7, 0x0e, 0x4c, 0x66, 0x1b, 0x63, 0xd6, 0x75 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous152 */ {
            /* {dccbe7e4-7750-466b-a557-5ea36c8ff24e} */
            { 0xdccbe7e4, 0x7750, 0x466b, { 0xa5, 0x57, 0x5e, 0xa3, 0x6c, 0x8f, 0xf2, 0x4e } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous242 */ {
            /* {5874af6d-5719-4e1b-b155-ef4eae7fcb32} */
            { 0x5874af6d, 0x5719, 0x4e1b, { 0xb1, 0x55, 0xef, 0x4e, 0xae, 0x7f, 0xcb, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous209 */ {
            /* {23ad3531-11d2-4e8e-805a-6a752e91681a} */
            { 0x23ad3531, 0x11d2, 0x4e8e, { 0x80, 0x5a, 0x6a, 0x75, 0x2e, 0x91, 0x68, 0x1a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous008 */ {
            /* {8cece922-9720-42de-b7db-7cef88cb07ca} */
            { 0x8cece922, 0x9720, 0x42de, { 0xb7, 0xdb, 0x7c, 0xef, 0x88, 0xcb, 0x07, 0xca } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous091 */ {
            /* {d0f9db12-249c-11d5-9905-001083010e9b} */
            { 0xd0f9db12, 0x249c, 0x11d5, { 0x99, 0x05, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous457 */ {
            /* {af7b24cb-893f-41bb-961f-5a69388e27c3} */
            { 0xaf7b24cb, 0x893f, 0x41bb, { 0x96, 0x1f, 0x5a, 0x69, 0x38, 0x8e, 0x27, 0xc3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous260 */ {
            /* {6ebc941a-f2ff-4d56-b3b6-f7d0b9d73344} */
            { 0x6ebc941a, 0xf2ff, 0x4d56, { 0xb3, 0xb6, 0xf7, 0xd0, 0xb9, 0xd7, 0x33, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous211 */ {
            /* {0c4f1ddc-1dd2-11b2-9d95-f2fdf113044b} */
            { 0xc4f1ddc, 0x1dd2, 0x11b2, { 0x9d, 0x95, 0xf2, 0xfd, 0xf1, 0x13, 0x04, 0x4b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous292 */ {
            /* {1dcc23b0-d4cb-11dc-9ad6-479d56d89593} */
            { 0x1dcc23b0, 0xd4cb, 0x11dc, { 0x9a, 0xd6, 0x47, 0x9d, 0x56, 0xd8, 0x95, 0x93 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous318 */ {
            /* {26a4a019-2827-4a89-a85c-5931a678823a} */
            { 0x26a4a019, 0x2827, 0x4a89, { 0xa8, 0x5c, 0x59, 0x31, 0xa6, 0x78, 0x82, 0x3a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous049 */ {
            /* {7fe6e18b-0da3-4056-bf3b-440ef3809e06} */
            { 0x7fe6e18b, 0xda3, 0x4056, { 0xbf, 0x3b, 0x44, 0x0e, 0xf3, 0x80, 0x9e, 0x06 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous071 */ {
            /* {215b2f62-46e2-4004-a3d1-6858e56c20f3} */
            { 0x215b2f62, 0x46e2, 0x4004, { 0xa3, 0xd1, 0x68, 0x58, 0xe5, 0x6c, 0x20, 0xf3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous024 */ {
            /* {56ebedd4-6ccf-48e8-bdae-adc77f044567} */
            { 0x56ebedd4, 0x6ccf, 0x48e8, { 0xbd, 0xae, 0xad, 0xc7, 0x7f, 0x04, 0x45, 0x67 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous336 */ {
            /* {e43b0010-04ba-4da6-b523-1f92580bc150} */
            { 0xe43b0010, 0x4ba, 0x4da6, { 0xb5, 0x23, 0x1f, 0x92, 0x58, 0x0b, 0xc1, 0x50 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous153 */ {
            /* {aea16cd0-f020-4138-b068-0716c4a15b5a} */
            { 0xaea16cd0, 0xf020, 0x4138, { 0xb0, 0x68, 0x07, 0x16, 0xc4, 0xa1, 0x5b, 0x5a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous417 */ {
            /* {58f089ee-512a-42d2-a935-d0c874128930} */
            { 0x58f089ee, 0x512a, 0x42d2, { 0xa9, 0x35, 0xd0, 0xc8, 0x74, 0x12, 0x89, 0x30 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous226 */ {
            /* {57972956-5718-42d2-8070-b3fc72212eaf} */
            { 0x57972956, 0x5718, 0x42d2, { 0x80, 0x70, 0xb3, 0xfc, 0x72, 0x21, 0x2e, 0xaf } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous098 */ {
            /* {0a698c44-3bff-11d4-9649-00c0ca135b4e} */
            { 0xa698c44, 0x3bff, 0x11d4, { 0x96, 0x49, 0x00, 0xc0, 0xca, 0x13, 0x5b, 0x4e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous471 */ {
            /* {7090524d-5574-4492-a77f-d8d558ced59d} */
            { 0x7090524d, 0x5574, 0x4492, { 0xa7, 0x7f, 0xd8, 0xd5, 0x58, 0xce, 0xd5, 0x9d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous251 */ {
            /* {63a69303-8a64-45a9-848c-d4e2792794e6} */
            { 0x63a69303, 0x8a64, 0x45a9, { 0x84, 0x8c, 0xd4, 0xe2, 0x79, 0x27, 0x94, 0xe6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous328 */ {
            /* {4399533d-08d1-458c-a87a-235f74451cfa} */
            { 0x4399533d, 0x8d1, 0x458c, { 0xa8, 0x7a, 0x23, 0x5f, 0x74, 0x45, 0x1c, 0xfa } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous324 */ {
            /* {1e668408-40aa-4185-991c-12671e2471dc} */
            { 0x1e668408, 0x40aa, 0x4185, { 0x99, 0x1c, 0x12, 0x67, 0x1e, 0x24, 0x71, 0xdc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous167 */ {
            /* {831f8f13-7aa8-485f-b02e-77c881cc5773} */
            { 0x831f8f13, 0x7aa8, 0x485f, { 0xb0, 0x2e, 0x77, 0xc8, 0x81, 0xcc, 0x57, 0x73 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous364 */ {
            /* {06beec76-a183-4d9f-85dd-085f26da565a} */
            { 0x6beec76, 0xa183, 0x4d9f, { 0x85, 0xdd, 0x08, 0x5f, 0x26, 0xda, 0x56, 0x5a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous031 */ {
            /* {1f34bc80-1bc7-11d6-a384-d705dd0746fc} */
            { 0x1f34bc80, 0x1bc7, 0x11d6, { 0xa3, 0x84, 0xd7, 0x05, 0xdd, 0x07, 0x46, 0xfc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous470 */ {
            /* {abfbb785-5a44-49cf-88db-2f300bf727c9} */
            { 0xabfbb785, 0x5a44, 0x49cf, { 0x88, 0xdb, 0x2f, 0x30, 0x0b, 0xf7, 0x27, 0xc9 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* URIFixup */ {
            /* {c6cf88b7-452e-47eb-bdc9-86e3561648ef} */
            { 0xc6cf88b7, 0x452e, 0x47eb, { 0xbd, 0xc9, 0x86, 0xe3, 0x56, 0x16, 0x48, 0xef } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* TypeAheadFind */ {
            /* {e7f70966-9a37-48d7-8aeb-35998f31090e} */
            { 0xe7f70966, 0x9a37, 0x48d7, { 0x8a, 0xeb, 0x35, 0x99, 0x8f, 0x31, 0x09, 0x0e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous133 */ {
            /* {62b778a6-bce3-456b-8c31-2865fbb68c91} */
            { 0x62b778a6, 0xbce3, 0x456b, { 0x8c, 0x31, 0x28, 0x65, 0xfb, 0xb6, 0x8c, 0x91 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous418 */ {
            /* {972d8d8f-f0da-11d4-9885-00c04fa0cf4b} */
            { 0x972d8d8f, 0xf0da, 0x11d4, { 0x98, 0x85, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Alerts */ {
            /* {a0ccaaf8-09da-44d8-b250-9ac3e93c8117} */
            { 0xa0ccaaf8, 0x9da, 0x44d8, { 0xb2, 0x50, 0x9a, 0xc3, 0xe9, 0x3c, 0x81, 0x17 } },
            { 0x2125 } /* "@mozilla.org/alerts-service;1" */,
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous339 */ {
            /* {84e11f80-ca55-11dd-ad8b-0800200c9a66} */
            { 0x84e11f80, 0xca55, 0x11dd, { 0xad, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous099 */ {
            /* {8b042e22-6f87-11d3-b3c8-00805f8a6670} */
            { 0x8b042e22, 0x6f87, 0x11d3, { 0xb3, 0xc8, 0x00, 0x80, 0x5f, 0x8a, 0x66, 0x70 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous314 */ {
            /* {3f78ada1-cba2-442a-82dd-d5fb300ddea7} */
            { 0x3f78ada1, 0xcba2, 0x442a, { 0x82, 0xdd, 0xd5, 0xfb, 0x30, 0x0d, 0xde, 0xa7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous185 */ {
            /* {b0054ef3-b096-483d-8242-4ee36b7b2115} */
            { 0xb0054ef3, 0xb096, 0x483d, { 0x82, 0x42, 0x4e, 0xe3, 0x6b, 0x7b, 0x21, 0x15 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous282 */ {
            /* {dc6c2976-0f73-4f1f-b9ff-3d72b4e28309} */
            { 0xdc6c2976, 0xf73, 0x4f1f, { 0xb9, 0xff, 0x3d, 0x72, 0xb4, 0xe2, 0x83, 0x09 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous304 */ {
            /* {895db6c7-dbdf-40ea-9f64-b175033243dc} */
            { 0x895db6c7, 0xdbdf, 0x40ea, { 0x9f, 0x64, 0xb1, 0x75, 0x03, 0x32, 0x43, 0xdc } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous354 */ {
            /* {948a0023-e3a7-11d2-96cf-0060b0fb9956} */
            { 0x948a0023, 0xe3a7, 0x11d2, { 0x96, 0xcf, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous000 */ {
            /* {7e4bb6ad-2fc4-4dc6-89ef-23e8e5ccf980} */
            { 0x7e4bb6ad, 0x2fc4, 0x4dc6, { 0x89, 0xef, 0x23, 0xe8, 0xe5, 0xcc, 0xf9, 0x80 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous301 */ {
            /* {1c978d25-b37f-43a8-a2d6-0c7a239ead87} */
            { 0x1c978d25, 0xb37f, 0x43a8, { 0xa2, 0xd6, 0x0c, 0x7a, 0x23, 0x9e, 0xad, 0x87 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous073 */ {
            /* {814f947a-52f7-41c9-94a1-3684797284ac} */
            { 0x814f947a, 0x52f7, 0x41c9, { 0x94, 0xa1, 0x36, 0x84, 0x79, 0x72, 0x84, 0xac } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous414 */ {
            /* {79a2b7cc-f05b-4605-bfa0-fac54f27eec8} */
            { 0x79a2b7cc, 0xf05b, 0x4605, { 0xbf, 0xa0, 0xfa, 0xc5, 0x4f, 0x27, 0xee, 0xc8 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous078 */ {
            /* {07611dc6-bf4d-4d8a-a64b-f3a5904dddc7} */
            { 0x7611dc6, 0xbf4d, 0x4d8a, { 0xa6, 0x4b, 0xf3, 0xa5, 0x90, 0x4d, 0xdd, 0xc7 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous002 */ {
            /* {5d0ce354-df01-421a-83fb-7ead0990c24e} */
            { 0x5d0ce354, 0xdf01, 0x421a, { 0x83, 0xfb, 0x7e, 0xad, 0x09, 0x90, 0xc2, 0x4e } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous422 */ {
            /* {fc886801-e768-11d4-9885-00c04fa0cf4b} */
            { 0xfc886801, 0xe768, 0x11d4, { 0x98, 0x85, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous088 */ {
            /* {92ae3ab2-8968-41b1-8709-b6123bceaf21} */
            { 0x92ae3ab2, 0x8968, 0x41b1, { 0x87, 0x09, 0xb6, 0x12, 0x3b, 0xce, 0xaf, 0x21 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous136 */ {
            /* {6ddb050c-0d04-11d4-986e-00c04fa0cf4a} */
            { 0x6ddb050c, 0xd04, 0x11d4, { 0x98, 0x6e, 0x00, 0xc0, 0x4f, 0xa0, 0xcf, 0x4a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous297 */ {
            /* {7d47b41d-7cc5-4882-b293-d8f3f3b48b46} */
            { 0x7d47b41d, 0x7cc5, 0x4882, { 0xb2, 0x93, 0xd8, 0xf3, 0xf3, 0xb4, 0x8b, 0x46 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous023 */ {
            /* {33d75835-722f-42c0-89cc-44f328e56a86} */
            { 0x33d75835, 0x722f, 0x42c0, { 0x89, 0xcc, 0x44, 0xf3, 0x28, 0xe5, 0x6a, 0x86 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous341 */ {
            /* {95d89e3e-a169-41a3-8e56-719978e15b12} */
            { 0x95d89e3e, 0xa169, 0x41a3, { 0x8e, 0x56, 0x71, 0x99, 0x78, 0xe1, 0x5b, 0x12 } },
            { 0x2143 } /* "@mozilla.org/xre/app-info;1" */,
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous252 */ {
            /* {6356aa16-7916-4215-a825-cbc2692ca87a} */
            { 0x6356aa16, 0x7916, 0x4215, { 0xa8, 0x25, 0xcb, 0xc2, 0x69, 0x2c, 0xa8, 0x7a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous321 */ {
            /* {e6156350-2be8-11db-a98b-0800200c9a66} */
            { 0xe6156350, 0x2be8, 0x11db, { 0xa9, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous406 */ {
            /* {d07f5195-e3d1-11d2-8acd-00105a1b8860} */
            { 0xd07f5195, 0xe3d1, 0x11d2, { 0x8a, 0xcd, 0x00, 0x10, 0x5a, 0x1b, 0x88, 0x60 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_VR_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous264 */ {
            /* {d9d75e86-8f17-4c57-993e-f738f0d86d42} */
            { 0xd9d75e86, 0x8f17, 0x4c57, { 0x99, 0x3e, 0xf7, 0x38, 0xf0, 0xd8, 0x6d, 0x42 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous169 */ {
            /* {4c39159c-cd90-4dd3-97a7-06af5e6d84c4} */
            { 0x4c39159c, 0xcd90, 0x4dd3, { 0x97, 0xa7, 0x06, 0xaf, 0x5e, 0x6d, 0x84, 0xc4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous132 */ {
            /* {36b63ef3-e0fa-4c49-9fd4-e065e85568f4} */
            { 0x36b63ef3, 0xe0fa, 0x4c49, { 0x9f, 0xd4, 0xe0, 0x65, 0xe8, 0x55, 0x68, 0xf4 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous285 */ {
            /* {dc185a77-ba88-4caa-8f16-465253f7599a} */
            { 0xdc185a77, 0xba88, 0x4caa, { 0x8f, 0x16, 0x46, 0x52, 0x53, 0xf7, 0x59, 0x9a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous473 */ {
            /* {48c345e7-9929-4f9a-a563-f478222dabcd} */
            { 0x48c345e7, 0x9929, 0x4f9a, { 0xa5, 0x63, 0xf4, 0x78, 0x22, 0x2d, 0xab, 0xcd } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous345 */ {
            /* {5573967d-f6cf-4c63-8e0e-9ac06e04d62b} */
            { 0x5573967d, 0xf6cf, 0x4c63, { 0x8e, 0x0e, 0x9a, 0xc0, 0x6e, 0x04, 0xd6, 0x2b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous022 */ {
            /* {11342911-3135-45a8-8d71-737a2b0ad469} */
            { 0x11342911, 0x3135, 0x45a8, { 0x8d, 0x71, 0x73, 0x7a, 0x2b, 0x0a, 0xd4, 0x69 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous421 */ {
            /* {fe0ff7c3-8e97-448b-9a8a-86afdb9fbbb6} */
            { 0xfe0ff7c3, 0x8e97, 0x448b, { 0x9a, 0x8a, 0x86, 0xaf, 0xdb, 0x9f, 0xbb, 0xb6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous366 */ {
            /* {841387c8-72e6-484b-9296-bf6eea80d58a} */
            { 0x841387c8, 0x72e6, 0x484b, { 0x92, 0x96, 0xbf, 0x6e, 0xea, 0x80, 0xd5, 0x8a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* AsyncShutdown */ {
            /* {35c496de-a115-475d-93b5-ffa3f3ae6fe3} */
            { 0x35c496de, 0xa115, 0x475d, { 0x93, 0xb5, 0xff, 0xa3, 0xf3, 0xae, 0x6f, 0xe3 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous302 */ {
            /* {7ad1b327-6dfa-46ec-9234-f2a620ea7e00} */
            { 0x7ad1b327, 0x6dfa, 0x46ec, { 0x92, 0x34, 0xf2, 0xa6, 0x20, 0xea, 0x7e, 0x00 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* UrlClassifierPrefixSet */ {
            /* {3d8579f0-75fa-4e00-ba41-38661d5b5d17} */
            { 0x3d8579f0, 0x75fa, 0x4e00, { 0xba, 0x41, 0x38, 0x66, 0x1d, 0x5b, 0x5d, 0x17 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous334 */ {
            /* {093c2356-4843-4c65-8709-d7dbcbbe7dfb} */
            { 0x93c2356, 0x4843, 0x4c65, { 0x87, 0x09, 0xd7, 0xdb, 0xcb, 0xbe, 0x7d, 0xfb } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous299 */ {
            /* {79a13656-a472-4713-b0e1-ab39a15cf790} */
            { 0x79a13656, 0xa472, 0x4713, { 0xb0, 0xe1, 0xab, 0x39, 0xa1, 0x5c, 0xf7, 0x90 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous394 */ {
            /* {acf8dc46-4a25-11d3-9890-006008962422} */
            { 0xacf8dc46, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous305 */ {
            /* {c11c21b2-71c9-4f87-a0f8-5e13f50495fd} */
            { 0xc11c21b2, 0x71c9, 0x4f87, { 0xa0, 0xf8, 0x5e, 0x13, 0xf5, 0x04, 0x95, 0xfd } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous129 */ {
            /* {cb9abbae-66b6-4609-8594-5c4ff300888e} */
            { 0xcb9abbae, 0x66b6, 0x4609, { 0x85, 0x94, 0x5c, 0x4f, 0xf3, 0x00, 0x88, 0x8e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous033 */ {
            /* {2dc08eaf-6eef-4394-b1df-a3a927c1290b} */
            { 0x2dc08eaf, 0x6eef, 0x4394, { 0xb1, 0xdf, 0xa3, 0xa9, 0x27, 0xc1, 0x29, 0x0b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous083 */ {
            /* {c1354898-e3fe-4602-88a7-c4520c21cb4e} */
            { 0xc1354898, 0xe3fe, 0x4602, { 0x88, 0xa7, 0xc4, 0x52, 0x0c, 0x21, 0xcb, 0x4e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* DownloadPlatform */ {
            /* {649a14c9-fe5c-48ec-9c85-00cad9ccf32e} */
            { 0x649a14c9, 0xfe5c, 0x48ec, { 0x9c, 0x85, 0x00, 0xca, 0xd9, 0xcc, 0xf3, 0x2e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous168 */ {
            /* {9879908a-2972-40c0-890b-a91dd7dfb954} */
            { 0x9879908a, 0x2972, 0x40c0, { 0x89, 0x0b, 0xa9, 0x1d, 0xd7, 0xdf, 0xb9, 0x54 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous148 */ {
            /* {b6ed3030-6183-11d3-a178-0050041caf44} */
            { 0xb6ed3030, 0x6183, 0x11d3, { 0xa1, 0x78, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous376 */ {
            /* {eb833911-4f49-4623-845f-e58a8e6de4c2} */
            { 0xeb833911, 0x4f49, 0x4623, { 0x84, 0x5f, 0xe5, 0x8a, 0x8e, 0x6d, 0xe4, 0xc2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous037 */ {
            /* {3fc8f04e-d719-43ca-9ad0-18ee320211f2} */
            { 0x3fc8f04e, 0xd719, 0x43ca, { 0x9a, 0xd0, 0x18, 0xee, 0x32, 0x02, 0x11, 0xf2 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous412 */ {
            /* {5ff24248-1dd2-11b2-8427-fbab44f29bc8} */
            { 0x5ff24248, 0x1dd2, 0x11b2, { 0x84, 0x27, 0xfb, 0xab, 0x44, 0xf2, 0x9b, 0xc8 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous329 */ {
            /* {9df8ef2b-94da-45c9-ab9f-132eb55fddf1} */
            { 0x9df8ef2b, 0x94da, 0x45c9, { 0xab, 0x9f, 0x13, 0x2e, 0xb5, 0x5f, 0xdd, 0xf1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous112 */ {
            /* {064d9cee-1dd2-11b2-83e3-d25ab0193c26} */
            { 0x64d9cee, 0x1dd2, 0x11b2, { 0x83, 0xe3, 0xd2, 0x5a, 0xb0, 0x19, 0x3c, 0x26 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous210 */ {
            /* {47402be2-e653-45d0-8daa-9f0dce0ac148} */
            { 0x47402be2, 0xe653, 0x45d0, { 0x8d, 0xaa, 0x9f, 0x0d, 0xce, 0x0a, 0xc1, 0x48 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous296 */ {
            /* {60a1f7c6-4ff9-4a42-84d3-5a185faa6f32} */
            { 0x60a1f7c6, 0x4ff9, 0x4a42, { 0x84, 0xd3, 0x5a, 0x18, 0x5f, 0xaa, 0x6f, 0x32 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous416 */ {
            /* {574ce83e-fe9f-4095-b85c-7909abbf7c37} */
            { 0x574ce83e, 0xfe9f, 0x4095, { 0xb8, 0x5c, 0x79, 0x09, 0xab, 0xbf, 0x7c, 0x37 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous030 */ {
            /* {a7f800e0-4306-11d4-98d0-001083010e9b} */
            { 0xa7f800e0, 0x4306, 0x11d4, { 0x98, 0xd0, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous348 */ {
            /* {8b1ae382-51a9-4972-b930-56977a57919d} */
            { 0x8b1ae382, 0x51a9, 0x4972, { 0xb9, 0x30, 0x56, 0x97, 0x7a, 0x57, 0x91, 0x9d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous409 */ {
            /* {7225c040-a9bf-11d3-a197-0050041caf44} */
            { 0x7225c040, 0xa9bf, 0x11d3, { 0xa1, 0x97, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* StreamTransport */ {
            /* {0885d4f8-f7b8-4cda-902e-94ba38bc256e} */
            { 0x885d4f8, 0xf7b8, 0x4cda, { 0x90, 0x2e, 0x94, 0xba, 0x38, 0xbc, 0x25, 0x6e } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous238 */ {
            /* {a6b2f8b0-7438-11ea-bc55-0242ac130003} */
            { 0xa6b2f8b0, 0x7438, 0x11ea, { 0xbc, 0x55, 0x02, 0x42, 0xac, 0x13, 0x00, 0x03 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous017 */ {
            /* {d8c9acee-dec5-11e4-8c75-1681e6b88ec1} */
            { 0xd8c9acee, 0xdec5, 0x11e4, { 0x8c, 0x75, 0x16, 0x81, 0xe6, 0xb8, 0x8e, 0xc1 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous385 */ {
            /* {acf8dc4a-4a25-11d3-9890-006008962422} */
            { 0xacf8dc4a, 0x4a25, 0x11d3, { 0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous200 */ {
            /* {b4f96c89-5238-450c-8bda-e12c26f1d150} */
            { 0xb4f96c89, 0x5238, 0x450c, { 0x8b, 0xda, 0xe1, 0x2c, 0x26, 0xf1, 0xd1, 0x50 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* CacheStorage */ {
            /* {ea70b098-5014-4e21-aee1-75e6b2c4b8e0} */
            { 0xea70b098, 0x5014, 0x4e21, { 0xae, 0xe1, 0x75, 0xe6, 0xb2, 0xc4, 0xb8, 0xe0 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous270 */ {
            /* {5b7047b4-fe17-4661-8e13-871402bc2023} */
            { 0x5b7047b4, 0xfe17, 0x4661, { 0x8e, 0x13, 0x87, 0x14, 0x02, 0xbc, 0x20, 0x23 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous338 */ {
            /* {bfd4a9d8-d886-4161-81ef-8868da114170} */
            { 0xbfd4a9d8, 0xd886, 0x4161, { 0x81, 0xef, 0x88, 0x68, 0xda, 0x11, 0x41, 0x70 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous353 */ {
            /* {bd57cee8-1dd1-11b2-9fe7-95cf4709aea3} */
            { 0xbd57cee8, 0x1dd1, 0x11b2, { 0x9f, 0xe7, 0x95, 0xcf, 0x47, 0x09, 0xae, 0xa3 } },
            { 0xffffffff },
            Module::ProcessSelector::MAIN_PROCESS_ONLY,
          },
  
          /* Anonymous383 */ {
            /* {669a9795-6ff7-4ed4-9150-c34ce2971b63} */
            { 0x669a9795, 0x6ff7, 0x4ed4, { 0x91, 0x50, 0xc3, 0x4c, 0xe2, 0x97, 0x1b, 0x63 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous436 */ {
            /* {62ec8731-55ad-4246-b2ea-f26c1fe19d2d} */
            { 0x62ec8731, 0x55ad, 0x4246, { 0xb2, 0xea, 0xf2, 0x6c, 0x1f, 0xe1, 0x9d, 0x2d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous178 */ {
            /* {66230b2b-17fa-4bd3-abf4-07986151022d} */
            { 0x66230b2b, 0x17fa, 0x4bd3, { 0xab, 0xf4, 0x07, 0x98, 0x61, 0x51, 0x02, 0x2d } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous201 */ {
            /* {7a6da992-dbce-4943-b463-5a2dd011fa1a} */
            { 0x7a6da992, 0xdbce, 0x4943, { 0xb4, 0x63, 0x5a, 0x2d, 0xd0, 0x11, 0xfa, 0x1a } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous085 */ {
            /* {20557898-1dd2-11b2-8f65-9c462ee2bc95} */
            { 0x20557898, 0x1dd2, 0x11b2, { 0x8f, 0x65, 0x9c, 0x46, 0x2e, 0xe2, 0xbc, 0x95 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous124 */ {
            /* {bdbe0555-fc3d-4f7b-9205-c309ceb2d641} */
            { 0xbdbe0555, 0xfc3d, 0x4f7b, { 0x92, 0x05, 0xc3, 0x09, 0xce, 0xb2, 0xd6, 0x41 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* History */ {
            /* {0937a705-91a6-417a-8292-b22eb10da86c} */
            { 0x937a705, 0x91a6, 0x417a, { 0x82, 0x92, 0xb2, 0x2e, 0xb1, 0x0d, 0xa8, 0x6c } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous142 */ {
            /* {969adfdf-7221-4419-aecf-05f8faf00c9b} */
            { 0x969adfdf, 0x7221, 0x4419, { 0xae, 0xcf, 0x05, 0xf8, 0xfa, 0xf0, 0x0c, 0x9b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous323 */ {
            /* {742ad274-34c5-43d1-a8b7-293eaf8962d6} */
            { 0x742ad274, 0x34c5, 0x43d1, { 0xa8, 0xb7, 0x29, 0x3e, 0xaf, 0x89, 0x62, 0xd6 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous266 */ {
            /* {47a45e5f-691e-4799-8686-14f8d3fc0f8b} */
            { 0x47a45e5f, 0x691e, 0x4799, { 0x86, 0x86, 0x14, 0xf8, 0xd3, 0xfc, 0x0f, 0x8b } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous106 */ {
            /* {245abae2-b947-4ded-a46d-9829d3cca462} */
            { 0x245abae2, 0xb947, 0x4ded, { 0xa4, 0x6d, 0x98, 0x29, 0xd3, 0xcc, 0xa4, 0x62 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous268 */ {
            /* {49e8d8ef-a713-492a-a3d2-5c9dad4ce2e5} */
            { 0x49e8d8ef, 0xa713, 0x492a, { 0xa3, 0xd2, 0x5c, 0x9d, 0xad, 0x4c, 0xe2, 0xe5 } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          },
  
          /* Anonymous126 */ {
            /* {b0ff4572-dae4-4bef-a092-83c1b88f6be9} */
            { 0xb0ff4572, 0xdae4, 0x4bef, { 0xa0, 0x92, 0x83, 0xc1, 0xb8, 0x8f, 0x6b, 0xe9 } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_SOCKET_PROCESS,
          },
  
          /* Anonymous368 */ {
            /* {cb6cdb94-e417-4601-b4a5-f991bf41453d} */
            { 0xcb6cdb94, 0xe417, 0x4601, { 0xb4, 0xa5, 0xf9, 0x91, 0xbf, 0x41, 0x45, 0x3d } },
            { 0xffffffff },
            Module::ProcessSelector::ALLOW_IN_GPU_RDD_AND_SOCKET_PROCESS,
          },
  
          /* Anonymous332 */ {
            /* {e35d5067-95bc-4029-8432-e8f1e431148d} */
            { 0xe35d5067, 0x95bc, 0x4029, { 0x84, 0x32, 0xe8, 0xf1, 0xe4, 0x31, 0x14, 0x8d } },
            { 0xffffffff },
            Module::ProcessSelector::ANY_PROCESS,
          }
};



const StaticModule*
ModuleByCID(const nsID& aKey)
{
  static const uint16_t BASES[] = {
       0,   2,   1,   1,   1,   0,   1,   1,   2,   1,   1,   1,   3,   0,   1,   1,
       6,   3,   1,   3,   1,   1,   0,   0,   1,   0,   1,   1,   1,   0,   1,   0,
       0,   0,   2,   1,   1,   0,   0,   0,   1,   1,   0,   0,   3,   1,   7,   1,
       0,   1,   0,   1,   1,   4,   0,   3,   1,   2,   0,   4,   7,   0,   3,   0,
       2,   1,   0,   0,   0,   0,   2,   1,   1,   3,   1,   2,   0,   1,   1,   0,
       0,   0,   8,   1,   1,   1,   0,   3,   3,   0,   1,   0,   0,   5,   3,   2,
       2,   0,   0,   0,   3,   7,   0,   1,   2,   2,   6,   1,  10,   0,   0,   2,
       2,   1,   1,   1,  15,   3,   3,   1,   4,   4,   0,   5,   2,   1,   0,   3,
       0,   1,   4,   0,   2,   5,   1,   1,   1,   4,   1,   0,   2,   1,   0,   0,
       1,   1,   0,   0,   0,   6,   2,   1,   0,   0,   0,   1,   1,   2,   0,   1,
      10,   1,   0,   0,   4,   3,  14,  28,   6,   0,   5,   0,   0,   0,   7,   8,
       4,   0,   0,   0,   1,   1,   2,   2,   2,   0,   1,   2,   5,   3,   3,  13,
       1,   6,   8,   1,   5,   0,   0,   0,   2,   0,   5,  18,  12,   2,   2,   2,
       2,   4,   8,   0,   3,   0,   1,   0,   0,   0,   1,   0,   1,   0,   3,   0,
       1,   3,   0,   1,   0,  22,   7,   0,   3,   1,   0,   0,   0,   0,   2,   5,
       1,   0,   0,   0,   2,   1,   0,   0,   0,   0,   0,   0,   7,   0,   2,   0,
       1,   0,   0,   1,   0,   1,   5,   3,   1,   1,   4,   3,   7,   2,   0,   3,
       3,   0,   0,  10,   6,   0,   9,   2,   0,   0,   2,   0,   3,   0,   2,   0,
       0,  19,   2,   1,   9,   0,   4,   4,   1,   1,   4,   0,   0,   3,   0,   2,
       2,   4,  11,   2,   0,   0,   2,   0,   1,   4,   4,   2,  19,   5,   5,   6,
      16,   2,  23,   0,   4,   0,   0,   0,   0,   3,   4,   1,   1,   0,   1,   0,
       0,   8,   2,   2,   0,  16,   6,   2,   2,   0,   1,  12,   0,  19,   1,   3,
       8,   1,   1,   0,   0,   0,   0,   1,   9,   1,   7,   2,   4,   9,   9,   0,
       0,  12,   9,   7,   4,   0,   0,   0,   2,   0,   0,   0,   2,   4,   1,   3,
       1,  19,   1,   3,   1,   0,   0,  27,  15,   2,   0,   9,   6,  15,   0,  24,
       2,   1,   0,   0,   0,  34,   5,   0,   7,  14,   0,   4,   0,  36,   0,   0,
       0,  17,   0,   9,   6,   2,   1,   3,  15,   0,   0,   7,  21,   4,   0,   0,
      35,   1,   0,   0,  14,   0,   5,   4,   0,   5,   0,   5,   9,   4,   0,  94,
       1,   0,  10,  36,   5,   5,   2,   2,  21,   3,   1,   1,   8,   0,   0,   0,
       0,   0,   8,   0,   0,   1,   0,  30,  23,   0,   6,  20,   0,  13,   0,   0,
       2,   4,   0,   0,   0, 117,   6,   0,  12,  91,  10, 107,   6,  34,   0,   6,
       0,   0,  98,   0,   0,   0,   3,   9, 161,  87,  33, 311, 118,   7, 870,2749,
  };
  

  const char* bytes = reinterpret_cast<const char*>(&aKey);
  size_t length = sizeof(nsID);
  auto& entry = mozilla::perfecthash::Lookup(bytes, length, BASES,
                                             gStaticModules);
  return entry.CID().Equals(aKey) && entry.Active() ? &entry : nullptr;
}

const ContractEntry gContractEntries[] = {
  
          {
            { 0x215f } /* "@mozilla.org/content-permission/prompt;1" */,
            ModuleID::Anonymous005,
          },
  
          {
            { 0x2188 } /* "@mozilla.org/appservices/logger;1" */,
            ModuleID::Anonymous232,
          },
  
          {
            { 0x21aa } /* "@mozilla.org/reputationservice/application-reputation-service;1" */,
            ModuleID::ApplicationReputation,
          },
  
          {
            { 0xc10 } /* "@mozilla.org/presentation-device/manager;1" */,
            ModuleID::Anonymous458,
          },
  
          {
            { 0x21ea } /* "@mozilla.org/uriloader/content-handler;1?type=image/x-icon" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x2225 } /* "@mozilla.org/profile/migrator;1?app=browser&type=chrome-dev" */,
            ModuleID::Anonymous011,
          },
  
          {
            { 0x2261 } /* "@mozilla.org/streamconv;1?from=application/octet-stream&to=*\/\*" */,
            ModuleID::Anonymous286,
          },
  
          {
            { 0x6d3 } /* "@mozilla.org/storage/vacuum;1" */,
            ModuleID::Anonymous240,
          },
  
          {
            { 0x22a0 } /* "@mozilla.org/security/random-generator;1" */,
            ModuleID::Anonymous223,
          },
  
          {
            { 0x22c9 } /* "@mozilla.org/appshell/appShellService;1" */,
            ModuleID::Anonymous413,
          },
  
          {
            { 0x22f1 } /* "@mozilla.org/updates/update-checker;1" */,
            ModuleID::Anonymous335,
          },
  
          {
            { 0x2317 } /* "@mozilla.org/autocomplete/controller;1" */,
            ModuleID::Anonymous246,
          },
  
          {
            { 0x233e } /* "@mozilla.org/security/hash;1" */,
            ModuleID::Anonymous217,
          },
  
          {
            { 0x235b } /* "@mozilla.org/embedcomp/rangefind;1" */,
            ModuleID::Anonymous342,
          },
  
          {
            { 0x20c2 } /* "@mozilla.org/widget/clipboard;1" */,
            ModuleID::Anonymous359,
          },
  
          {
            { 0x237e } /* "@mozilla.org/layout/contentserializer;1?mimetype=text/xml" */,
            ModuleID::Anonymous445,
          },
  
          {
            { 0x23b8 } /* "@mozilla.org/network/protocol/about;1?what=history" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x23eb } /* "@mozilla.org/browser/favicon-service;1" */,
            ModuleID::Anonymous288,
          },
  
          {
            { 0x2412 } /* "@mozilla.org/memory-info-dumper;1" */,
            ModuleID::Anonymous378,
          },
  
          {
            { 0x2434 } /* "@mozilla.org/network/async-stream-copier;1" */,
            ModuleID::Anonymous116,
          },
  
          {
            { 0x245f } /* "@mozilla.org/login-manager/crypto/SDR;1" */,
            ModuleID::Anonymous282,
          },
  
          {
            { 0x2487 } /* "@mozilla.org/presentation/control-service;1" */,
            ModuleID::Anonymous072,
          },
  
          {
            { 0x24b3 } /* "@mozilla.org/supports-PRUint32;1" */,
            ModuleID::Anonymous395,
          },
  
          {
            { 0x24d4 } /* "@mozilla.org/network/mime-hdrparam;1" */,
            ModuleID::Anonymous139,
          },
  
          {
            { 0x24f9 } /* "@mozilla.org/network/url-parser;1?auth=maybe" */,
            ModuleID::Anonymous172,
          },
  
          {
            { 0x2526 } /* "@mozilla.org/toolkit/osfile/native-internals;1" */,
            ModuleID::Anonymous251,
          },
  
          {
            { 0x2555 } /* "@mozilla.org/addon-web-api/manager;1" */,
            ModuleID::Anonymous331,
          },
  
          {
            { 0x257a } /* "@mozilla.org/network/url-parser;1?auth=no" */,
            ModuleID::Anonymous173,
          },
  
          {
            { 0x25a4 } /* "@mozilla.org/identity/crypto-service;1" */,
            ModuleID::Anonymous233,
          },
  
          {
            { 0x25cb } /* "@mozilla.org/network/buffered-output-stream;1" */,
            ModuleID::Anonymous122,
          },
  
          {
            { 0x25f9 } /* "@mozilla.org/intl/localeservice;1" */,
            ModuleID::Anonymous093,
          },
  
          {
            { 0x261b } /* "@mozilla.org/layout/contentserializer;1?mimetype=text/html" */,
            ModuleID::Anonymous447,
          },
  
          {
            { 0x2656 } /* "@mozilla.org/dom/rtcdtmfsender;1" */,
            ModuleID::Anonymous045,
          },
  
          {
            { 0x2677 } /* "@mozilla.org/network/idn-service;1" */,
            ModuleID::Anonymous133,
          },
  
          {
            { 0x269a } /* "@mozilla.org/io/arraybuffer-input-stream;1" */,
            ModuleID::Anonymous113,
          },
  
          {
            { 0x26c5 } /* "@mozilla.org/security/oskeystore;1" */,
            ModuleID::Anonymous226,
          },
  
          {
            { 0x26e8 } /* "@mozilla.org/gfx/printsettings-service;1" */,
            ModuleID::Anonymous366,
          },
  
          {
            { 0x2711 } /* "@mozilla.org/thirdpartyutil;1" */,
            ModuleID::ThirdPartyUtil,
          },
  
          {
            { 0x272f } /* "@mozilla.org/extensions/storage/internal/sync-area;1" */,
            ModuleID::Anonymous269,
          },
  
          {
            { 0x2764 } /* "@mozilla.org/autocomplete/search;1?name=search-autocomplete" */,
            ModuleID::Anonymous309,
          },
  
          {
            { 0x27a0 } /* "@mozilla.org/geolocation/provider;1" */,
            ModuleID::Anonymous077,
          },
  
          {
            { 0x27c4 } /* "@mozilla.org/widget/image-to-gdk-pixbuf;1" */,
            ModuleID::Anonymous356,
          },
  
          {
            { 0x27ee } /* "@mozilla.org/docloaderservice;1" */,
            ModuleID::DocLoader,
          },
  
          {
            { 0x280e } /* "@mozilla.org/network/protocol/about;1?what=certerror" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x2843 } /* "@mozilla.org/gfx/info;1" */,
            ModuleID::GfxInfo,
          },
  
          {
            { 0x285b } /* "@mozilla.org/network/util;1" */,
            ModuleID::IO,
          },
  
          {
            { 0x2877 } /* "@mozilla.org/url-classifier/listmanager;1" */,
            ModuleID::Anonymous319,
          },
  
          {
            { 0x28a1 } /* "@mozilla.org/widget/transferable;1" */,
            ModuleID::Anonymous357,
          },
  
          {
            { 0x28c4 } /* "@mozilla.org/browser/tagging-service;1" */,
            ModuleID::Anonymous291,
          },
  
          {
            { 0x28eb } /* "@mozilla.org/network/protocol/about;1?what=loginsimportreport" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0xd56 } /* "@mozilla.org/updates/update-service-stub;1" */,
            ModuleID::Anonymous336,
          },
  
          {
            { 0xa24 } /* "@mozilla.org/dom/payments/payment-complete-action-response;1" */,
            ModuleID::Anonymous065,
          },
  
          {
            { 0x2929 } /* "@mozilla.org/supports-float;1" */,
            ModuleID::Anonymous387,
          },
  
          {
            { 0x2947 } /* "@mozilla.org/dom/workers/workerdebuggermanager;1" */,
            ModuleID::Anonymous436,
          },
  
          {
            { 0x2978 } /* "@mozilla.org/updates/update-manager;1" */,
            ModuleID::Anonymous334,
          },
  
          {
            { 0x299e } /* "@mozilla.org/memory-reporter-manager;1" */,
            ModuleID::Anonymous379,
          },
  
          {
            { 0x29c5 } /* "@mozilla.org/network/udp-filter-handler;1?name=stun" */,
            ModuleID::Anonymous042,
          },
  
          {
            { 0x7a } /* "@mozilla.org/content-viewers/http-index-format" */,
            ModuleID::Anonymous323,
          },
  
          {
            { 0x29f9 } /* "@mozilla.org/observer-service;1" */,
            ModuleID::Anonymous406,
          },
  
          {
            { 0x2a19 } /* "@mozilla.org/serviceworkers/manager;1" */,
            ModuleID::ServiceWorkerManager,
          },
  
          {
            { 0x2a3f } /* "@mozilla.org/alert-notification;1" */,
            ModuleID::AlertNotification,
          },
  
          {
            { 0x2a61 } /* "@mozilla.org/autocomplete/simple-result;1" */,
            ModuleID::Anonymous247,
          },
  
          {
            { 0x2a8b } /* "@mozilla.org/url-classifier/jslib;1" */,
            ModuleID::Anonymous318,
          },
  
          {
            { 0x2aaf } /* "@mozilla.org/network/network-link-service;1" */,
            ModuleID::Anonymous196,
          },
  
          {
            { 0x2adb } /* "@mozilla.org/jsdebugger;1" */,
            ModuleID::Anonymous101,
          },
  
          {
            { 0x2af5 } /* "@mozilla.org/network/protocol;1?name=default" */,
            ModuleID::Anonymous025,
          },
  
          {
            { 0x2b22 } /* "@mozilla.org/docshell/structured-clone-container;1" */,
            ModuleID::Anonymous430,
          },
  
          {
            { 0x2b55 } /* "@mozilla.org/streamconv;1?from=uncompressed&to=x-gzip" */,
            ModuleID::Anonymous109,
          },
  
          {
            { 0x2b8b } /* "@mozilla.org/chrome/chrome-registry;1" */,
            ModuleID::ChromeRegistry,
          },
  
          {
            { 0x2bb1 } /* "@mozilla.org/network/protocol/about;1?what=glean" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x2be2 } /* "@mozilla.org/network/protocol;1?name=moz-page-thumb" */,
            ModuleID::Anonymous154,
          },
  
          {
            { 0x2c16 } /* "@mozilla.org/network/atomic-file-output-stream;1" */,
            ModuleID::Anonymous117,
          },
  
          {
            { 0x2c47 } /* "@mozilla.org/remote/agent;1" */,
            ModuleID::Anonymous204,
          },
  
          {
            { 0x2c63 } /* "@mozilla.org/network/serialization-helper;1" */,
            ModuleID::Anonymous161,
          },
  
          {
            { 0x2c8f } /* "@mozilla.org/url-classifier/exception-list-service;1" */,
            ModuleID::Anonymous202,
          },
  
          {
            { 0x2cc4 } /* "@mozilla.org/network/protocol/about;1?what=protections" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x2cfb } /* "@mozilla.org/http-sfv-service;1" */,
            ModuleID::Anonymous192,
          },
  
          {
            { 0x2d1b } /* "@mozilla.org/url-classifier/utils;1" */,
            ModuleID::UrlClassifierUtils,
          },
  
          {
            { 0x2d3f } /* "@mozilla.org/processtools-service;1" */,
            ModuleID::Anonymous299,
          },
  
          {
            { 0x2d63 } /* "@mozilla.org/intl/stringbundle;1" */,
            ModuleID::StringBundle,
          },
  
          {
            { 0x2d84 } /* "@mozilla.org/streamconv;1?from=text/ftp-dir&to=application/http-index-format" */,
            ModuleID::Anonymous180,
          },
  
          {
            { 0x981 } /* "@mozilla.org/dom/payments/payment-address;1" */,
            ModuleID::Anonymous063,
          },
  
          {
            { 0x2dd1 } /* "@mozilla.org/dom/peerconnectionobserver;1" */,
            ModuleID::Anonymous044,
          },
  
          {
            { 0x2dfb } /* "@mozilla.org/gfx/screenmanager;1" */,
            ModuleID::Anonymous350,
          },
  
          {
            { 0x2e1c } /* "@mozilla.org/scriptablebase64encoder;1" */,
            ModuleID::Anonymous381,
          },
  
          {
            { 0x2e43 } /* "@mozilla.org/find/find_service;1" */,
            ModuleID::FindService,
          },
  
          {
            { 0x2e64 } /* "@mozilla.org/toolkit/components/mdnsresponder/dns-sd;1" */,
            ModuleID::Anonymous198,
          },
  
          {
            { 0x2e9b } /* "@mozilla.org/system-alerts-service;1" */,
            ModuleID::Anonymous339,
          },
  
          {
            { 0x2ec0 } /* "@mozilla.org/xul/xulstore;1" */,
            ModuleID::Anonymous325,
          },
  
          {
            { 0x2edc } /* "@mozilla.org/streamconv;1?from=uncompressed&to=deflate" */,
            ModuleID::Anonymous109,
          },
  
          {
            { 0x2f13 } /* "@mozilla.org/netwerk/cache-storage-service;1" */,
            ModuleID::CacheStorage,
          },
  
          {
            { 0x2f40 } /* "@mozilla.org/partitioning/exception-list-service;1" */,
            ModuleID::Anonymous244,
          },
  
          {
            { 0x2f73 } /* "@mozilla.org/typeaheadfind;1" */,
            ModuleID::TypeAheadFind,
          },
  
          {
            { 0x2f90 } /* "@mozilla.org/streamconv;1?from=application/vnd.mozilla.json.view&to=*\/\*" */,
            ModuleID::Anonymous017,
          },
  
          {
            { 0x2fd8 } /* "@mozilla.org/embedcomp/dialogparam;1" */,
            ModuleID::Anonymous346,
          },
  
          {
            { 0x2ffd } /* "@mozilla.org/streamconv;1?from=gzip&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x3031 } /* "@mozilla.org/streamconv;1?from=br&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x3063 } /* "@mozilla.org/network/protocol/about;1?what=certificate" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x309a } /* "@mozilla.org/streamconv;1?from=application/x-unknown-content-type&to=*\/\*" */,
            ModuleID::Anonymous177,
          },
  
          {
            { 0x30e3 } /* "@mozilla.org/toolkit/finalizationwitness;1" */,
            ModuleID::Anonymous250,
          },
  
          {
            { 0x310e } /* "@mozilla.org/network/protocol/about;1?what=pocket-saved" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x3146 } /* "@mozilla.org/network/protocol/about;1?what=memory" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x3178 } /* "@mozilla.org/geolocation;1" */,
            ModuleID::Anonymous441,
          },
  
          {
            { 0xc4f } /* "@mozilla.org/presentation/presentationservice;1" */,
            ModuleID::Anonymous459,
          },
  
          {
            { 0x3193 } /* "@mozilla.org/embedcomp/default-tooltiptextprovider;1" */,
            ModuleID::Anonymous317,
          },
  
          {
            { 0x2d } /* "@mozilla.org/content/document-loader-factory;1" */,
            ModuleID::Anonymous422,
          },
  
          {
            { 0x31c8 } /* "@mozilla.org/intl/converter-output-stream;1" */,
            ModuleID::Anonymous097,
          },
  
          {
            { 0x31f4 } /* "@mozilla.org/dom/rtcsessiondescription;1" */,
            ModuleID::Anonymous047,
          },
  
          {
            { 0x4ca } /* "@mozilla.org/addons/content-policy;1" */,
            ModuleID::AddonContentPolicy,
          },
  
          {
            { 0x321d } /* "@mozilla.org/uriloader/handler-service;1" */,
            ModuleID::Anonymous027,
          },
  
          {
            { 0x3246 } /* "@mozilla.org/network/binary-detector;1" */,
            ModuleID::Anonymous120,
          },
  
          {
            { 0x326d } /* "@mozilla.org/gfx/printsession;1" */,
            ModuleID::Anonymous365,
          },
  
          {
            { 0x328d } /* "@mozilla.org/network/protocol;1?name=blob" */,
            ModuleID::Anonymous038,
          },
  
          {
            { 0x32b7 } /* "@mozilla.org/network/network-connectivity-service;1" */,
            ModuleID::Anonymous141,
          },
  
          {
            { 0x32eb } /* "@mozilla.org/network/server-socket;1" */,
            ModuleID::Anonymous162,
          },
  
          {
            { 0xee5 } /* "@mozilla.org/fork-server-launcher;1" */,
            ModuleID::Anonymous100,
          },
  
          {
            { 0xbda } /* "@mozilla.org/crashservice;1" */,
            ModuleID::Anonymous262,
          },
  
          {
            { 0x3310 } /* "@mozilla.org/referrer-info;1" */,
            ModuleID::Anonymous451,
          },
  
          {
            { 0x332d } /* "@mozilla.org/supports-double;1" */,
            ModuleID::Anonymous386,
          },
  
          {
            { 0x334c } /* "@mozilla.org/toolkit/shell-service;1" */,
            ModuleID::Anonymous001,
          },
  
          {
            { 0x3371 } /* "@mozilla.org/network/protocol/about;1?what=checkerboard" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x33a9 } /* "@mozilla.org/network/throttlequeue;1" */,
            ModuleID::Anonymous169,
          },
  
          {
            { 0x5d4 } /* "@mozilla.org/cspservice;1" */,
            ModuleID::Anonymous427,
          },
  
          {
            { 0x33ce } /* "@mozilla.org/intl/ospreferences;1" */,
            ModuleID::Anonymous094,
          },
  
          {
            { 0x33f0 } /* "@mozilla.org/network/protocol;1?name=http" */,
            ModuleID::Anonymous151,
          },
  
          {
            { 0x341a } /* "@mozilla.org/process/environment;1" */,
            ModuleID::Anonymous411,
          },
  
          {
            { 0x404 } /* "@mozilla.org/commandlinehandler/general-startup;1?type=remote" */,
            ModuleID::Anonymous205,
          },
  
          {
            { 0x343d } /* "@mozilla.org/network/protocol;1?name=javascript" */,
            ModuleID::Anonymous453,
          },
  
          {
            { 0x346d } /* "@mozilla.org/toolkit/app-startup;1" */,
            ModuleID::AppStartup,
          },
  
          {
            { 0x63f } /* "@mozilla.org/media/sniffer;1" */,
            ModuleID::Anonymous272,
          },
  
          {
            { 0x3490 } /* "@mozilla.org/eventlistenerservice;1" */,
            ModuleID::Anonymous437,
          },
  
          {
            { 0x34b4 } /* "@mozilla.org/network/stream-loader;1" */,
            ModuleID::Anonymous168,
          },
  
          {
            { 0x364 } /* "@mozilla.org/browser/clh;1" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x34d9 } /* "@mozilla.org/autocomplete/search;1?name=login-doorhanger-username" */,
            ModuleID::Anonymous285,
          },
  
          {
            { 0x351b } /* "@mozilla.org/network/socket-transport-service;1" */,
            ModuleID::SocketTransport,
          },
  
          {
            { 0x354b } /* "@mozilla.org/network/effective-tld-service;1" */,
            ModuleID::Anonymous129,
          },
  
          {
            { 0x3578 } /* "@mozilla.org/ssservice;1" */,
            ModuleID::Anonymous225,
          },
  
          {
            { 0x3591 } /* "@mozilla.org/dom/localStorage-manager;1" */,
            ModuleID::Anonymous432,
          },
  
          {
            { 0x35b9 } /* "@mozilla.org/network/udp-socket;1" */,
            ModuleID::Anonymous171,
          },
  
          {
            { 0x2143 } /* "@mozilla.org/xre/app-info;1" */,
            ModuleID::Anonymous341,
          },
  
          {
            { 0x35db } /* "@mozilla.org/dom/rtpreceiver;1" */,
            ModuleID::Anonymous052,
          },
  
          {
            { 0x35fa } /* "@mozilla.org/network-info-service;1" */,
            ModuleID::Anonymous195,
          },
  
          {
            { 0x361e } /* "@mozilla.org/xre/directory-provider;1" */,
            ModuleID::Anonymous345,
          },
  
          {
            { 0x3644 } /* "@mozilla.org/binaryoutputstream;1" */,
            ModuleID::Anonymous372,
          },
  
          {
            { 0x3666 } /* "@mozilla.org/layout/contentserializer;1?mimetype=application/xhtml+xml" */,
            ModuleID::Anonymous446,
          },
  
          {
            { 0x9ca } /* "@mozilla.org/dom/payments/payment-canmake-action-response;1" */,
            ModuleID::Anonymous064,
          },
  
          {
            { 0x36ad } /* "@mozilla.org/uriloader/content-handler;1?type=image/svg+xml" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x36e9 } /* "@mozilla.org/profile/migrator;1?app=browser&type=firefox" */,
            ModuleID::Anonymous009,
          },
  
          {
            { 0x3722 } /* "@mozilla.org/dom/peerconnectionstatic;1" */,
            ModuleID::Anonymous050,
          },
  
          {
            { 0xe58 } /* "@mozilla.org/synthspeechdispatcher;1" */,
            ModuleID::Anonymous054,
          },
  
          {
            { 0x374a } /* "@mozilla.org/dom/dom-request-service;1" */,
            ModuleID::Anonymous431,
          },
  
          {
            { 0xf1d } /* "@mozilla.org/updates/update-sync-manager;1" */,
            ModuleID::Anonymous254,
          },
  
          {
            { 0x3771 } /* "@mozilla.org/supports-PRUint16;1" */,
            ModuleID::Anonymous394,
          },
  
          {
            { 0x3792 } /* "@mozilla.org/crashmanager;1" */,
            ModuleID::Anonymous263,
          },
  
          {
            { 0x37ae } /* "@mozilla.org/streamconv;1?from=compress&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x37e6 } /* "@mozilla.org/network/well-known-opportunistic-utils;1" */,
            ModuleID::Anonymous200,
          },
  
          {
            { 0x381c } /* "@mozilla.org/gsettings-service;1" */,
            ModuleID::Anonymous338,
          },
  
          {
            { 0x383d } /* "@mozilla.org/security/certoverride;1" */,
            ModuleID::Anonymous222,
          },
  
          {
            { 0x624 } /* "@mozilla.org/image/cache;1" */,
            ModuleID::Anonymous083,
          },
  
          {
            { 0x3862 } /* "@mozilla.org/toolkit/captive-detector;1" */,
            ModuleID::Anonymous257,
          },
  
          {
            { 0x388a } /* "@mozilla.org/thread-manager;1" */,
            ModuleID::Anonymous400,
          },
  
          {
            { 0x38a8 } /* "@mozilla.org/preferences-service;1" */,
            ModuleID::Anonymous111,
          },
  
          {
            { 0x38cb } /* "@mozilla.org/xpcom/version-comparator;1" */,
            ModuleID::Anonymous402,
          },
  
          {
            { 0x38f3 } /* "@mozilla.org/clear-data-service;1" */,
            ModuleID::Anonymous259,
          },
  
          {
            { 0xb26 } /* "@mozilla.org/places/expiration;1" */,
            ModuleID::Anonymous293,
          },
  
          {
            { 0x3915 } /* "@mozilla.org/presentation-device/multicastdns-provider;1" */,
            ModuleID::Anonymous073,
          },
  
          {
            { 0x394e } /* "@mozilla.org/network/protocol/about;1?what=tabcrashed" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x3984 } /* "@mozilla.org/uriloader;1" */,
            ModuleID::URILoader,
          },
  
          {
            { 0x399d } /* "@mozilla.org/nsClientAuthDialogs;1" */,
            ModuleID::Anonymous206,
          },
  
          {
            { 0x39c0 } /* "@mozilla.org/streamconv;1?from=application/http-index-format&to=text/html" */,
            ModuleID::Anonymous176,
          },
  
          {
            { 0x3a0a } /* "@mozilla.org/synth-voice-registry;1" */,
            ModuleID::Anonymous471,
          },
  
          {
            { 0x3a2e } /* "@mozilla.org/thumbnails/pagethumbs-service;1" */,
            ModuleID::Anonymous315,
          },
  
          {
            { 0x3a5b } /* "@mozilla.org/network/protocol/about;1?what=webrtc" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x54b } /* "@mozilla.org/image-blocker-content-policy;1" */,
            ModuleID::Anonymous084,
          },
  
          {
            { 0x3a8d } /* "@mozilla.org/jsinspector;1" */,
            ModuleID::Anonymous018,
          },
  
          {
            { 0x3aa8 } /* "@mozilla.org/dom/slow-script-debug;1" */,
            ModuleID::Anonymous034,
          },
  
          {
            { 0x3acd } /* "@mozilla.org/load-context-info-factory;1" */,
            ModuleID::Anonymous114,
          },
  
          {
            { 0x3af6 } /* "@mozilla.org/security/nsCertTree;1" */,
            ModuleID::Anonymous229,
          },
  
          {
            { 0x3b19 } /* "@mozilla.org/streamconv;1?from=application/pdf&to=*\/\*" */,
            ModuleID::Anonymous286,
          },
  
          {
            { 0x3b4f } /* "@mozilla.org/supports-char;1" */,
            ModuleID::Anonymous385,
          },
  
          {
            { 0x3b6c } /* "@mozilla.org/uriloader/content-handler;1?type=text/html" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x3ba4 } /* "@mozilla.org/content-dispatch-chooser;1" */,
            ModuleID::Anonymous332,
          },
  
          {
            { 0x3bcc } /* "@mozilla.org/network/protocol;1?name=moz-gio" */,
            ModuleID::Anonymous199,
          },
  
          {
            { 0xba9 } /* "@mozilla.org/toolkit/crashmonitor;1" */,
            ModuleID::Anonymous264,
          },
  
          {
            { 0x3bf9 } /* "@mozilla.org/dom/createofferrequest;1" */,
            ModuleID::Anonymous053,
          },
  
          {
            { 0x3c1f } /* "@mozilla.org/network/input-stream-pump;1" */,
            ModuleID::Anonymous137,
          },
  
          {
            { 0x3c48 } /* "@mozilla.org/nsTokenDialogs;1" */,
            ModuleID::Anonymous206,
          },
  
          {
            { 0x3c66 } /* "@mozilla.org/websocketevent/service;1" */,
            ModuleID::Anonymous469,
          },
  
          {
            { 0x3c8c } /* "@mozilla.org/browser/synced-bookmarks-merger;1" */,
            ModuleID::Anonymous297,
          },
  
          {
            { 0x3cbb } /* "@mozilla.org/network/authprompt-adapter-factory;1" */,
            ModuleID::Anonymous300,
          },
  
          {
            { 0xac5 } /* "@mozilla.org/dom/payments/payment-show-action-response;1" */,
            ModuleID::Anonymous067,
          },
  
          {
            { 0xa77 } /* "@mozilla.org/dom/payments/payment-request-service;1" */,
            ModuleID::Anonymous066,
          },
  
          {
            { 0x3ced } /* "@mozilla.org/gfx/devicecontextspec;1" */,
            ModuleID::Anonymous363,
          },
  
          {
            { 0x3d12 } /* "@mozilla.org/network/protocol;1?name=about" */,
            ModuleID::Anonymous147,
          },
  
          {
            { 0x3d3d } /* "@mozilla.org/embedcomp/prompt-collection;1" */,
            ModuleID::Anonymous013,
          },
  
          {
            { 0x3d68 } /* "@mozilla.org/autocomplete/search;1?name=places-tag-autocomplete" */,
            ModuleID::Anonymous292,
          },
  
          {
            { 0x3da8 } /* "@mozilla.org/nsTokenPasswordDialogs;1" */,
            ModuleID::Anonymous206,
          },
  
          {
            { 0x3dce } /* "@mozilla.org/dom/rtpsender;1" */,
            ModuleID::Anonymous051,
          },
  
          {
            { 0x3deb } /* "@mozilla.org/network/protocol/about;1?what=reader" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x3e1d } /* "@mozilla.org/network/protocol;1?name=jar" */,
            ModuleID::Anonymous105,
          },
  
          {
            { 0x3e46 } /* "@mozilla.org/network/protocol/about;1?what=support" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x3e79 } /* "@mozilla.org/storagestream;1" */,
            ModuleID::Anonymous383,
          },
  
          {
            { 0x3e96 } /* "@mozilla.org/addons/integration;1" */,
            ModuleID::Anonymous328,
          },
  
          {
            { 0x3eb8 } /* "@mozilla.org/streamconv;1?from=multipart/x-mixed-replace&to=*\/\*" */,
            ModuleID::Anonymous179,
          },
  
          {
            { 0x3ef8 } /* "@mozilla.org/network/protocol/about;1?what=newpreferences" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x3f32 } /* "@mozilla.org/applicationchooser;1" */,
            ModuleID::Anonymous358,
          },
  
          {
            { 0x3f54 } /* "@mozilla.org/weave/service;1" */,
            ModuleID::Anonymous236,
          },
  
          {
            { 0x3f71 } /* "@mozilla.org/network/protocol/about;1?what=blocked" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x3fa4 } /* "@mozilla.org/network/protocol;1?name=moz" */,
            ModuleID::Anonymous275,
          },
  
          {
            { 0x3fcd } /* "@mozilla.org/network/protocol/about;1?what=buildconfig" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4004 } /* "@mozilla.org/network/protocol/about;1?what=processes" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4039 } /* "@mozilla.org/widget/dragservice;1" */,
            ModuleID::Anonymous361,
          },
  
          {
            { 0x577 } /* "@mozilla.org/mixedcontentblocker;1" */,
            ModuleID::Anonymous450,
          },
  
          {
            { 0x20fd } /* "@mozilla.org/embedcomp/window-watcher;1" */,
            ModuleID::Anonymous344,
          },
  
          {
            { 0x405b } /* "@mozilla.org/inspector/deep-tree-walker;1" */,
            ModuleID::Anonymous443,
          },
  
          {
            { 0x4085 } /* "@mozilla.org/network/protocol/about;1?what=url-classifier" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x477 } /* "@mozilla.org/toolkit/default-clh;1" */,
            ModuleID::Anonymous260,
          },
  
          {
            { 0x314 } /* "@mozilla.org/base/telemetry-controller-content;1" */,
            ModuleID::Anonymous312,
          },
  
          {
            { 0x40bf } /* "@mozilla.org/consoleservice;1" */,
            ModuleID::Anonymous374,
          },
  
          {
            { 0x40dd } /* "@mozilla.org/toolkit/URLFormatterService;1" */,
            ModuleID::Anonymous321,
          },
  
          {
            { 0x4108 } /* "@mozilla.org/network/protocol/about;1?what=printpreview" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4140 } /* "@mozilla.org/autocomplete/search;1?name=login-doorhanger-password" */,
            ModuleID::Anonymous285,
          },
  
          {
            { 0x4182 } /* "@mozilla.org/network/incremental-download;1" */,
            ModuleID::Anonymous134,
          },
  
          {
            { 0x41ae } /* "@mozilla.org/uriclassifierservice" */,
            ModuleID::UrlClassifierDB,
          },
  
          {
            { 0x41d0 } /* "@mozilla.org/hash-property-bag;1" */,
            ModuleID::Anonymous375,
          },
  
          {
            { 0x41f1 } /* "@mozilla.org/url-classifier/channel-classifier-service;1" */,
            ModuleID::Anonymous201,
          },
  
          {
            { 0x422a } /* "@mozilla.org/scriptable-content-iterator;1" */,
            ModuleID::Anonymous463,
          },
  
          {
            { 0x4255 } /* "@mozilla.org/url-classifier/hashcompleter;1" */,
            ModuleID::Anonymous320,
          },
  
          {
            { 0x51b } /* "@mozilla.org/embedding/browser/content-policy;1" */,
            ModuleID::Anonymous248,
          },
  
          {
            { 0x4281 } /* "@mozilla.org/startupcacheinfo;1" */,
            ModuleID::Anonymous238,
          },
  
          {
            { 0x42a1 } /* "@mozilla.org/push/Service;1" */,
            ModuleID::Anonymous075,
          },
  
          {
            { 0x42bd } /* "@mozilla.org/security/x509certdb;1" */,
            ModuleID::Anonymous215,
          },
  
          {
            { 0x42e0 } /* "@mozilla.org/nsGeneratingKeypairInfoDialogs;1" */,
            ModuleID::Anonymous206,
          },
  
          {
            { 0x430e } /* "@mozilla.org/network/protocol/about;1?what=devtools" */,
            ModuleID::Anonymous019,
          },
  
          {
            { 0x4342 } /* "@mozilla.org/sidebar;1" */,
            ModuleID::Anonymous310,
          },
  
          {
            { 0x4359 } /* "@mozilla.org/uriloader/content-handler;1?type=text/css" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x4390 } /* "@mozilla.org/uriloader/content-handler;1?type=image/png" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x43c8 } /* "@mozilla.org/appshell/component/browser-status-filter;1" */,
            ModuleID::Anonymous252,
          },
  
          {
            { 0x4400 } /* "@mozilla.org/push/PushManager;1" */,
            ModuleID::Anonymous074,
          },
  
          {
            { 0x4420 } /* "@mozilla.org/network/protocol-proxy-service;1" */,
            ModuleID::Anonymous143,
          },
  
          {
            { 0xd96 } /* "@mozilla.org/updates/timer-manager;1" */,
            ModuleID::Anonymous316,
          },
  
          {
            { 0x444e } /* "@mozilla.org/network/protocol/about;1?what=srcdoc" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4480 } /* "@mozilla.org/async-shutdown-service;1" */,
            ModuleID::AsyncShutdown,
          },
  
          {
            { 0x44a6 } /* "@mozilla.org/permissionmanager;1" */,
            ModuleID::PermissionManager,
          },
  
          {
            { 0x44c7 } /* "@mozilla.org/notificationStorage;1" */,
            ModuleID::Anonymous057,
          },
  
          {
            { 0x44ea } /* "@mozilla.org/uriloader/external-helper-app-service;1" */,
            ModuleID::Anonymous030,
          },
  
          {
            { 0x451f } /* "@mozilla.org/streamconv;1?from=deflate&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x4556 } /* "@mozilla.org/dom/rtcstatsreport;1" */,
            ModuleID::Anonymous049,
          },
  
          {
            { 0x4578 } /* "@mozilla.org/nsCertificateDialogs;1" */,
            ModuleID::Anonymous206,
          },
  
          {
            { 0x459c } /* "@mozilla.org/tools/profiler;1" */,
            ModuleID::Anonymous347,
          },
  
          {
            { 0xddb } /* "@mozilla.org/fxaccounts/push;1" */,
            ModuleID::Anonymous234,
          },
  
          {
            { 0x45ba } /* "@mozilla.org/expandedprincipal;1" */,
            ModuleID::Anonymous438,
          },
  
          {
            { 0x45db } /* "@mozilla.org/security/certstorage;1" */,
            ModuleID::Anonymous228,
          },
  
          {
            { 0x45ff } /* "@mozilla.org/profile/migrator;1?app=browser&type=chromium" */,
            ModuleID::Anonymous008,
          },
  
          {
            { 0x4639 } /* "@mozilla.org/libjar/zip-reader-cache;1" */,
            ModuleID::Anonymous108,
          },
  
          {
            { 0x4660 } /* "@mozilla.org/focus-manager;1" */,
            ModuleID::Anonymous439,
          },
  
          {
            { 0x467d } /* "@mozilla.org/content/plugin/document-loader-factory;1" */,
            ModuleID::Anonymous423,
          },
  
          {
            { 0x4ef } /* "@mozilla.org/data-document-content-policy;1" */,
            ModuleID::Anonymous428,
          },
  
          {
            { 0x46b3 } /* "@mozilla.org/intl/scriptableunicodeconverter" */,
            ModuleID::Anonymous098,
          },
  
          {
            { 0x46e0 } /* "@mozilla.org/browser/nav-bookmarks-service;1" */,
            ModuleID::Anonymous289,
          },
  
          {
            { 0x470d } /* "@mozilla.org/network/protocol/about;1?what=telemetry" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4742 } /* "@mozilla.org/network/background-file-saver;1?mode=outputstream" */,
            ModuleID::Anonymous118,
          },
  
          {
            { 0x4781 } /* "@mozilla.org/presentation/mockedsockettransport;1" */,
            ModuleID::Anonymous069,
          },
  
          {
            { 0x47b3 } /* "@mozilla.org/network/protocol/about;1?what=logo" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x47e3 } /* "@mozilla.org/network/http-auth-manager;1" */,
            ModuleID::Anonymous132,
          },
  
          {
            { 0x480c } /* "@mozilla.org/mediaManagerService;1" */,
            ModuleID::Anonymous449,
          },
  
          {
            { 0x482f } /* "@mozilla.org/image/encoder;2?type=image/bmp" */,
            ModuleID::Anonymous087,
          },
  
          {
            { 0x485b } /* "@mozilla.org/network/protocol/about;1?what=preferences" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x4892 } /* "@mozilla.org/toolkit/filewatcher/native-file-watcher;1" */,
            ModuleID::NativeFileWatcher,
          },
  
          {
            { 0x48c9 } /* "@mozilla.org/cspcontext;1" */,
            ModuleID::Anonymous426,
          },
  
          {
            { 0x48e3 } /* "@mozilla.org/network/protocol;1?name=chrome" */,
            ModuleID::Anonymous373,
          },
  
          {
            { 0x490f } /* "@mozilla.org/devicesensors;1" */,
            ModuleID::Anonymous429,
          },
  
          {
            { 0x492c } /* "@mozilla.org/network/protocol;1?name=view-source" */,
            ModuleID::Anonymous157,
          },
  
          {
            { 0x495d } /* "@mozilla.org/gfx/printerlist;1" */,
            ModuleID::Anonymous367,
          },
  
          {
            { 0x38a } /* "@mozilla.org/devtools/startup-clh;1" */,
            ModuleID::Anonymous020,
          },
  
          {
            { 0x497c } /* "@mozilla.org/dom/sdb-connection;1" */,
            ModuleID::Anonymous435,
          },
  
          {
            { 0x499e } /* "@mozilla.org/process/util;1" */,
            ModuleID::Anonymous380,
          },
  
          {
            { 0x49ba } /* "@mozilla.org/storage/activity-service;1" */,
            ModuleID::Anonymous466,
          },
  
          {
            { 0x49e2 } /* "@mozilla.org/webspeech/service;1?name=online" */,
            ModuleID::Anonymous472,
          },
  
          {
            { 0x4a0f } /* "@mozilla.org/pref-localizedstring;1" */,
            ModuleID::Anonymous112,
          },
  
          {
            { 0x4a33 } /* "@mozilla.org/network/protocol/about;1?what=newtab" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x4a65 } /* "@mozilla.org/network/application-cache-service;1" */,
            ModuleID::Anonymous115,
          },
  
          {
            { 0x4a96 } /* "@mozilla.org/url-classifier/dbservice;1" */,
            ModuleID::UrlClassifierDB,
          },
  
          {
            { 0x4abe } /* "@mozilla.org/network/protocol;1?name=moz-fonttable" */,
            ModuleID::Anonymous037,
          },
  
          {
            { 0x4af1 } /* "@mozilla.org/presentation/datachanneltransport;1" */,
            ModuleID::Anonymous070,
          },
  
          {
            { 0x4b22 } /* "@mozilla.org/network/protocol/about;1?what=ion" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x4b51 } /* "@mozilla.org/security/nssversion;1" */,
            ModuleID::Anonymous209,
          },
  
          {
            { 0x4b74 } /* "@mozilla.org/network/protocol;1?name=indexeddb" */,
            ModuleID::Anonymous076,
          },
  
          {
            { 0x4ba3 } /* "@mozilla.org/streamconv;1?from=uncompressed&to=rawdeflate" */,
            ModuleID::Anonymous109,
          },
  
          {
            { 0x4bdd } /* "@mozilla.org/push/Notifier;1" */,
            ModuleID::Anonymous462,
          },
  
          {
            { 0x4bfa } /* "@mozilla.org/jsctypes;1" */,
            ModuleID::Anonymous265,
          },
  
          {
            { 0x4c12 } /* "@mozilla.org/base/telemetry-startup;1" */,
            ModuleID::Anonymous311,
          },
  
          {
            { 0x4c38 } /* "@mozilla.org/dom/sessionStorage-manager;1" */,
            ModuleID::Anonymous433,
          },
  
          {
            { 0x4c62 } /* "@mozilla.org/uriloader/content-handler;1?type=image/bmp" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x4c9a } /* "@mozilla.org/supports-PRTime;1" */,
            ModuleID::Anonymous393,
          },
  
          {
            { 0x4cb9 } /* "@mozilla.org/txttohtmlconv;1" */,
            ModuleID::Anonymous181,
          },
  
          {
            { 0x4cd6 } /* "@mozilla.org/parental-controls-service;1" */,
            ModuleID::Anonymous255,
          },
  
          {
            { 0x4cff } /* "@mozilla.org/system-proxy-settings;1" */,
            ModuleID::Anonymous340,
          },
  
          {
            { 0x4d24 } /* "@mozilla.org/supports-PRInt64;1" */,
            ModuleID::Anonymous392,
          },
  
          {
            { 0x4d44 } /* "@mozilla.org/cookieService;1" */,
            ModuleID::Anonymous191,
          },
  
          {
            { 0x4d61 } /* "@mozilla.org/gfx/fontenumerator;1" */,
            ModuleID::Anonymous082,
          },
  
          {
            { 0x687 } /* "@mozilla.org/content-blocking-telemetry-service;1" */,
            ModuleID::Anonymous245,
          },
  
          {
            { 0x4d83 } /* "@mozilla.org/autocomplete/search;1?name=form-history" */,
            ModuleID::Anonymous304,
          },
  
          {
            { 0x4db8 } /* "@mozilla.org/filepicker;1" */,
            ModuleID::Anonymous353,
          },
  
          {
            { 0x4dd2 } /* "@mozilla.org/streamconv;1?from=application/vnd.mozilla.webext.unlocalized&to=text/css" */,
            ModuleID::Anonymous322,
          },
  
          {
            { 0x4e28 } /* "@mozilla.org/streamconv;1?from=x-gzip&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x4e5e } /* "@mozilla.org/network/stream-transport-service;1" */,
            ModuleID::StreamTransport,
          },
  
          {
            { 0x4e8e } /* "@mozilla.org/geolocation/mls-provider;1" */,
            ModuleID::Anonymous077,
          },
  
          {
            { 0x4eb6 } /* "@mozilla.org/browser/search-service;1" */,
            ModuleID::Anonymous308,
          },
  
          {
            { 0x44c } /* "@mozilla.org/browser/final-clh;1" */,
            ModuleID::Anonymous003,
          },
  
          {
            { 0x4edc } /* "@mozilla.org/network/protocol/about;1?what=serviceworkers" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4f16 } /* "@mozilla.org/appshell/window-mediator;1" */,
            ModuleID::Anonymous414,
          },
  
          {
            { 0x4f3e } /* "@mozilla.org/streamconv;1?from=application/octet-stream&to=text/html" */,
            ModuleID::Anonymous286,
          },
  
          {
            { 0x4f83 } /* "@mozilla.org/network/protocol/about;1?what=license" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4fb6 } /* "@mozilla.org/network/protocol/about;1?what=crashes" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x4fe9 } /* "@mozilla.org/streamconv;1?from=x-compress&to=uncompressed" */,
            ModuleID::Anonymous178,
          },
  
          {
            { 0x5023 } /* "@mozilla.org/accessibilityService;1" */,
            ModuleID::Anonymous474,
          },
  
          {
            { 0x5047 } /* "@mozilla.org/browser/browserglue;1" */,
            ModuleID::Anonymous004,
          },
  
          {
            { 0x506a } /* "@mozilla.org/toolkit/glean;1" */,
            ModuleID::Anonymous271,
          },
  
          {
            { 0x5087 } /* "@mozilla.org/widget/htmlformatconverter;1" */,
            ModuleID::Anonymous354,
          },
  
          {
            { 0x50b1 } /* "@mozilla.org/securityconsole/message;1" */,
            ModuleID::Anonymous382,
          },
  
          {
            { 0x2cd } /* "@mozilla.org/satchel/form-fill-controller;1" */,
            ModuleID::Anonymous304,
          },
  
          {
            { 0x50d8 } /* "@mozilla.org/uriloader/content-handler;1?type=application/xhtml+xml" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x511c } /* "@mozilla.org/network/tls-server-socket;1" */,
            ModuleID::Anonymous170,
          },
  
          {
            { 0x5145 } /* "@mozilla.org/network/default-uri-mutator;1" */,
            ModuleID::Anonymous165,
          },
  
          {
            { 0x5170 } /* "@mozilla.org/security/osreauthenticator;1" */,
            ModuleID::Anonymous227,
          },
  
          {
            { 0x519a } /* "@mozilla.org/offlinecacheupdate-service;1" */,
            ModuleID::OfflineCacheUpdate,
          },
  
          {
            { 0x51c4 } /* "@mozilla.org/uriloader/local-handler-app;1" */,
            ModuleID::Anonymous028,
          },
  
          {
            { 0x51ef } /* "@mozilla.org/timer;1" */,
            ModuleID::Anonymous412,
          },
  
          {
            { 0x5204 } /* "@mozilla.org/url-classifier/streamupdater;1" */,
            ModuleID::UrlClassifierStreamUpdater,
          },
  
          {
            { 0x5230 } /* "@mozilla.org/network/protocol/about;1?what=compat" */,
            ModuleID::Anonymous015,
          },
  
          {
            { 0x5262 } /* "@mozilla.org/image/tools;1" */,
            ModuleID::Anonymous086,
          },
  
          {
            { 0xc93 } /* "@mozilla.org/purge-tracker-service;1" */,
            ModuleID::Anonymous243,
          },
  
          {
            { 0x527d } /* "@mozilla.org/uriloader/content-handler;1?type=text/xml" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x52b4 } /* "@mozilla.org/streamconv;1?from=application/pdf&to=text/html" */,
            ModuleID::Anonymous286,
          },
  
          {
            { 0x52f0 } /* "@mozilla.org/principal;1" */,
            ModuleID::Anonymous461,
          },
  
          {
            { 0x70e } /* "@mozilla.org/places/databaseUtilsIdleMaintenance;1" */,
            ModuleID::Anonymous294,
          },
  
          {
            { 0x5309 } /* "@mozilla.org/embedcomp/prompt-service;1" */,
            ModuleID::Anonymous302,
          },
  
          {
            { 0x5331 } /* "@mozilla.org/image/encoder;2?type=image/jpeg" */,
            ModuleID::Anonymous089,
          },
  
          {
            { 0x535e } /* "@mozilla.org/webnavigation-info;1" */,
            ModuleID::Anonymous029,
          },
  
          {
            { 0xb6d } /* "@mozilla.org/readconfig;1" */,
            ModuleID::Anonymous079,
          },
  
          {
            { 0x5380 } /* "@mozilla.org/supports-interface-pointer;1" */,
            ModuleID::Anonymous388,
          },
  
          {
            { 0x53aa } /* "@mozilla.org/security/pk11tokendb;1" */,
            ModuleID::Anonymous212,
          },
  
          {
            { 0x53ce } /* "@mozilla.org/network/protocol;1?name=resource" */,
            ModuleID::Anonymous156,
          },
  
          {
            { 0x53fc } /* "@mozilla.org/network/protocol/about;1?what=studies" */,
            ModuleID::Anonymous276,
          },
  
          {
            { 0x542f } /* "@mozilla.org/supports-PRInt32;1" */,
            ModuleID::Anonymous391,
          },
  
          {
            { 0x544f } /* "@mozilla.org/parentprocessmessagemanager;1" */,
            ModuleID::Anonymous456,
          },
  
          {
            { 0x608 } /* "@mozilla.org/image/loader;1" */,
            ModuleID::Anonymous083,
          },
  
          {
            { 0x547a } /* "@mozilla.org/security/transportsecurityinfo;1" */,
            ModuleID::Anonymous224,
          },
  
          {
            { 0x54a8 } /* "@mozilla.org/network/protocol/about;1?what=crashcontent" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x54e0 } /* "@mozilla.org/network/protocol;1?name=ws" */,
            ModuleID::Anonymous158,
          },
  
          {
            { 0x5508 } /* "@mozilla.org/prompter;1" */,
            ModuleID::Anonymous301,
          },
  
          {
            { 0x5520 } /* "@mozilla.org/uriloader/content-handler;1?type=image/gif" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x5558 } /* "@mozilla.org/network/protocol/about;1?what=home" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x5588 } /* "@mozilla.org/network/url-parser;1?auth=yes" */,
            ModuleID::Anonymous174,
          },
  
          {
            { 0x55b3 } /* "@mozilla.org/ipc/processselector;1" */,
            ModuleID::Anonymous033,
          },
  
          {
            { 0x55d6 } /* "@mozilla.org/network/protocol/about;1?what=robots" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x5608 } /* "@mozilla.org/parserutils;1" */,
            ModuleID::Anonymous457,
          },
  
          {
            { 0x5623 } /* "@mozilla.org/supports-cstring;1" */,
            ModuleID::Anonymous384,
          },
  
          {
            { 0x5643 } /* "@mozilla.org/layout/contentserializer;1?mimetype=application/vnd.mozilla.xul+xml" */,
            ModuleID::Anonymous445,
          },
  
          {
            { 0x5694 } /* "@mozilla.org/security/clientAuthRememberService;1" */,
            ModuleID::Anonymous216,
          },
  
          {
            { 0x56c6 } /* "@mozilla.org/network/protocol/about;1?what=mozilla" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x56f9 } /* "@mozilla.org/childprocessmessagemanager;1" */,
            ModuleID::Anonymous421,
          },
  
          {
            { 0x5723 } /* "@mozilla.org/widget/useridleservice;1" */,
            ModuleID::Anonymous362,
          },
  
          {
            { 0x5749 } /* "@mozilla.org/login-manager/autocompletesearch;1" */,
            ModuleID::Anonymous279,
          },
  
          {
            { 0x5779 } /* "@mozilla.org/network/protocol;1?name=https" */,
            ModuleID::Anonymous152,
          },
  
          {
            { 0x57a4 } /* "@mozilla.org/tracking-db-service;1" */,
            ModuleID::Anonymous241,
          },
  
          {
            { 0x57c7 } /* "@mozilla.org/network/protocol/about;1?what=addons" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x57f9 } /* "@mozilla.org/security/hmac;1" */,
            ModuleID::Anonymous218,
          },
  
          {
            { 0x5816 } /* "@mozilla.org/profile/migrator;1?app=browser&type=chrome" */,
            ModuleID::Anonymous007,
          },
  
          {
            { 0x584e } /* "@mozilla.org/login-manager/storage/json;1" */,
            ModuleID::Anonymous283,
          },
  
          {
            { 0x5878 } /* "@mozilla.org/network/simple-uri-mutator;1" */,
            ModuleID::Anonymous164,
          },
  
          {
            { 0x58a2 } /* "@mozilla.org/security/sdr;1" */,
            ModuleID::Anonymous211,
          },
  
          {
            { 0x58be } /* "@mozilla.org/colorpicker;1" */,
            ModuleID::Anonymous352,
          },
  
          {
            { 0x58d9 } /* "@mozilla.org/net/osfileconstantsservice;1" */,
            ModuleID::Anonymous452,
          },
  
          {
            { 0x5903 } /* "@mozilla.org/widget/clipboardhelper;1" */,
            ModuleID::Anonymous360,
          },
  
          {
            { 0x5929 } /* "@mozilla.org/webvttParserWrapper;1" */,
            ModuleID::Anonymous056,
          },
  
          {
            { 0x594c } /* "@mozilla.org/login-manager/loginInfo;1" */,
            ModuleID::Anonymous281,
          },
  
          {
            { 0x5973 } /* "@mozilla.org/uriloader/web-handler-app;1" */,
            ModuleID::Anonymous348,
          },
  
          {
            { 0x599c } /* "@mozilla.org/network/protocol/about;1?what=plugins" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x59cf } /* "@mozilla.org/intl/texttosuburi;1" */,
            ModuleID::Anonymous099,
          },
  
          {
            { 0x59f0 } /* "@mozilla.org/uriloader/content-handler;1?type=text/rdf" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x5a27 } /* "@mozilla.org/message-loop;1" */,
            ModuleID::Anonymous370,
          },
  
          {
            { 0x7b4 } /* "@mozilla.org/devtools/jsonview-sniffer;1" */,
            ModuleID::Anonymous016,
          },
  
          {
            { 0x5a43 } /* "@mozilla.org/presentation/datachanneltransportbuilder;1" */,
            ModuleID::Anonymous071,
          },
  
          {
            { 0x5a7b } /* "@mozilla.org/dom/rtcicecandidate;1" */,
            ModuleID::Anonymous046,
          },
  
          {
            { 0x5a9e } /* "@mozilla.org/peerconnection;1" */,
            ModuleID::Anonymous040,
          },
  
          {
            { 0x5abc } /* "@mozilla.org/zipwriter;1" */,
            ModuleID::Anonymous110,
          },
  
          {
            { 0x5ad5 } /* "@mozilla.org/network/protocol;1?name=data" */,
            ModuleID::Anonymous148,
          },
  
          {
            { 0x5aff } /* "@mozilla.org/network/protocol/about;1?what=credits" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x5b32 } /* "@mozilla.org/uriloader/content-handler;1?type=image/jpeg" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x5b6b } /* "@mozilla.org/layout/contentserializer;1?mimetype=image/svg+xml" */,
            ModuleID::Anonymous445,
          },
  
          {
            { 0x5baa } /* "@mozilla.org/network/tcp-filter-handler;1?name=stun" */,
            ModuleID::Anonymous041,
          },
  
          {
            { 0x5bde } /* "@mozilla.org/network/simple-stream-listener;1" */,
            ModuleID::Anonymous163,
          },
  
          {
            { 0x5c0c } /* "@mozilla.org/uriloader/content-handler;1?type=application/x-xpinstall" */,
            ModuleID::Anonymous330,
          },
  
          {
            { 0x5c52 } /* "@mozilla.org/docshell/uri-fixup-info;1" */,
            ModuleID::Anonymous023,
          },
  
          {
            { 0x5c79 } /* "@mozilla.org/network/protocol/about;1?what=welcome" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x5cac } /* "@mozilla.org/network/cache-storage-service;1" */,
            ModuleID::CacheStorage,
          },
  
          {
            { 0x5cd9 } /* "@mozilla.org/globalmessagemanager;1" */,
            ModuleID::Anonymous442,
          },
  
          {
            { 0x5cfd } /* "@mozilla.org/toolkit/shutdown-terminator;1" */,
            ModuleID::Anonymous256,
          },
  
          {
            { 0x5d28 } /* "@mozilla.org/cookieJarSettings;1" */,
            ModuleID::Anonymous193,
          },
  
          {
            { 0x5d49 } /* "@mozilla.org/libjar/zip-reader;1" */,
            ModuleID::Anonymous104,
          },
  
          {
            { 0x5d6a } /* "@mozilla.org/network/standard-url-mutator;1" */,
            ModuleID::Anonymous166,
          },
  
          {
            { 0x5d96 } /* "@mozilla.org/network/protocol/about;1?what=logins" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x5dc8 } /* "@mozilla.org/base/telemetry;1" */,
            ModuleID::Anonymous313,
          },
  
          {
            { 0x5de6 } /* "@mozilla.org/prefetch-service;1" */,
            ModuleID::Prefetch,
          },
  
          {
            { 0xccb } /* "@mozilla.org/satchel/form-history-startup;1" */,
            ModuleID::Anonymous307,
          },
  
          {
            { 0x5e06 } /* "@mozilla.org/dom/browser-element-api;1" */,
            ModuleID::Anonymous035,
          },
  
          {
            { 0x5e2d } /* "@mozilla.org/jsreflect;1" */,
            ModuleID::Anonymous303,
          },
  
          {
            { 0x5e46 } /* "@mozilla.org/widget/appshell/gtk;1" */,
            ModuleID::Anonymous349,
          },
  
          {
            { 0x5e69 } /* "@mozilla.org/supports-PRUint8;1" */,
            ModuleID::Anonymous397,
          },
  
          {
            { 0x5e89 } /* "@mozilla.org/network/protocol;1?name=page-icon" */,
            ModuleID::Anonymous296,
          },
  
          {
            { 0x5eb8 } /* "@mozilla.org/network/http-activity-distributor;1" */,
            ModuleID::HttpActivityDistributor,
          },
  
          {
            { 0x5ee9 } /* "@mozilla.org/network/background-file-saver;1?mode=streamlistener" */,
            ModuleID::Anonymous119,
          },
  
          {
            { 0xe21 } /* "@mozilla.org/fakesynth;1" */,
            ModuleID::Anonymous055,
          },
  
          {
            { 0x5f2a } /* "@mozilla.org/security/keyobject;1" */,
            ModuleID::Anonymous219,
          },
  
          {
            { 0x5f4c } /* "@mozilla.org/supports-string;1" */,
            ModuleID::Anonymous398,
          },
  
          {
            { 0x5f6b } /* "@mozilla.org/login-manager/authprompter;1" */,
            ModuleID::Anonymous280,
          },
  
          {
            { 0x5f95 } /* "@mozilla.org/ospermissionrequest;1" */,
            ModuleID::Anonymous026,
          },
  
          {
            { 0x59a } /* "@mozilla.org/no-data-protocol-content-policy;1" */,
            ModuleID::Anonymous454,
          },
  
          {
            { 0x5fb8 } /* "@mozilla.org/presentation/presentationtcpsessiontransport;1" */,
            ModuleID::Anonymous460,
          },
  
          {
            { 0x5ff4 } /* "@mozilla.org/supports-PRUint64;1" */,
            ModuleID::Anonymous396,
          },
  
          {
            { 0x6015 } /* "@mozilla.org/satchel/form-autocomplete;1" */,
            ModuleID::Anonymous305,
          },
  
          {
            { 0x603e } /* "@mozilla.org/toolkit/profile-migrator;1" */,
            ModuleID::Anonymous006,
          },
  
          {
            { 0x6066 } /* "@mozilla.org/transfer;1" */,
            ModuleID::Anonymous267,
          },
  
          {
            { 0x607e } /* "@mozilla.org/cascade-filter;1" */,
            ModuleID::Anonymous258,
          },
  
          {
            { 0x609c } /* "@mozilla.org/uriloader/content-handler;1?type=application/http-index-format" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x60e8 } /* "@mozilla.org/network/cache-service;1" */,
            ModuleID::Anonymous123,
          },
  
          {
            { 0x610d } /* "@mozilla.org/streamconv;1?from=uncompressed&to=gzip" */,
            ModuleID::Anonymous109,
          },
  
          {
            { 0x6141 } /* "@mozilla.org/network/file-output-stream;1" */,
            ModuleID::Anonymous131,
          },
  
          {
            { 0x616b } /* "@mozilla.org/network/protocol/about;1?what=pocket-signup" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x755 } /* "@mozilla.org/dom/quota-manager-service;1" */,
            ModuleID::Anonymous434,
          },
  
          {
            { 0x61a4 } /* "@mozilla.org/eventsourceevent/service;1" */,
            ModuleID::Anonymous470,
          },
  
          {
            { 0x61cc } /* "@mozilla.org/content/style-sheet-service;1" */,
            ModuleID::Anonymous424,
          },
  
          {
            { 0x61f7 } /* "@mozilla.org/toolkit/components/mdnsresponder/dns-info;1" */,
            ModuleID::Anonymous197,
          },
  
          {
            { 0x6230 } /* "@mozilla.org/network/protocol;1?name=dot" */,
            ModuleID::Anonymous266,
          },
  
          {
            { 0x6259 } /* "@mozilla.org/security/keyobjectfactory;1" */,
            ModuleID::Anonymous220,
          },
  
          {
            { 0x6282 } /* "@mozilla.org/sandbox/sandbox-settings;1" */,
            ModuleID::Anonymous230,
          },
  
          {
            { 0x62aa } /* "@mozilla.org/streamConverters;1" */,
            ModuleID::Anonymous175,
          },
  
          {
            { 0x62ca } /* "@mozilla.org/network/protocol;1?name=file" */,
            ModuleID::Anonymous149,
          },
  
          {
            { 0x62f4 } /* "@mozilla.org/nullprincipal;1" */,
            ModuleID::Anonymous455,
          },
  
          {
            { 0x6311 } /* "@mozilla.org/network/protocol;1?name=ftp" */,
            ModuleID::Anonymous150,
          },
  
          {
            { 0x633a } /* "@mozilla.org/spellchecker/engine;1" */,
            ModuleID::Anonymous080,
          },
  
          {
            { 0x635d } /* "@mozilla.org/addons/installtrigger;1" */,
            ModuleID::Anonymous329,
          },
  
          {
            { 0x6382 } /* "@mozilla.org/widget/printdialog-service;1" */,
            ModuleID::Anonymous364,
          },
  
          {
            { 0x63ac } /* "@mozilla.org/network/dns-service;1" */,
            ModuleID::Anonymous126,
          },
  
          {
            { 0x63cf } /* "@mozilla.org/mime;1" */,
            ModuleID::Anonymous030,
          },
  
          {
            { 0x63e3 } /* "@mozilla.org/toolkit/download-platform;1" */,
            ModuleID::DownloadPlatform,
          },
  
          {
            { 0x640c } /* "@mozilla.org/io/string-input-stream;1" */,
            ModuleID::Anonymous410,
          },
  
          {
            { 0x6432 } /* "@mozilla.org/layout/content-policy;1" */,
            ModuleID::Anonymous444,
          },
  
          {
            { 0x6457 } /* "@mozilla.org/content/dropped-link-handler;1" */,
            ModuleID::Anonymous031,
          },
  
          {
            { 0xe97 } /* "@mozilla.org/browser/nav-history-service;1" */,
            ModuleID::Anonymous290,
          },
  
          {
            { 0x6483 } /* "@mozilla.org/moz/jssubscript-loader;1" */,
            ModuleID::Anonymous102,
          },
  
          {
            { 0x64a9 } /* "@mozilla.org/sound;1" */,
            ModuleID::Anonymous355,
          },
  
          {
            { 0x8a4 } /* "@mozilla.org/dom/payments/general-change-details;1" */,
            ModuleID::Anonymous060,
          },
  
          {
            { 0x64be } /* "@mozilla.org/network/protocol/about;1?what=policies" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x64f2 } /* "@mozilla.org/intl/converter-input-stream;1" */,
            ModuleID::Anonymous096,
          },
  
          {
            { 0x651d } /* "@mozilla.org/helperapplauncherdialog;1" */,
            ModuleID::Anonymous326,
          },
  
          {
            { 0x6544 } /* "@mozilla.org/browser/shell-service;1" */,
            ModuleID::Anonymous001,
          },
  
          {
            { 0x6569 } /* "@mozilla.org/array;1" */,
            ModuleID::Anonymous405,
          },
  
          {
            { 0x657e } /* "@mozilla.org/main-process-singleton;1" */,
            ModuleID::Anonymous298,
          },
  
          {
            { 0x65a4 } /* "@mozilla.org/file/local;1" */,
            ModuleID::Anonymous377,
          },
  
          {
            { 0x65be } /* "@mozilla.org/network/protocol/about;1?what=blank" */,
            ModuleID::Anonymous144,
          },
  
          {
            { 0x65ef } /* "@mozilla.org/login-manager/storage/default;1" */,
            ModuleID::Anonymous283,
          },
  
          {
            { 0x661c } /* "@mozilla.org/consoleAPI-storage;1" */,
            ModuleID::Anonymous036,
          },
  
          {
            { 0x663e } /* "@mozilla.org/xpcom/memory-service;1" */,
            ModuleID::Anonymous369,
          },
  
          {
            { 0x6662 } /* "@mozilla.org/xre/runtime;1" */,
            ModuleID::Anonymous341,
          },
  
          {
            { 0x667d } /* "@mozilla.org/network/mime-input-stream;1" */,
            ModuleID::Anonymous140,
          },
  
          {
            { 0x66a6 } /* "@mozilla.org/network/protocol/about;1?what=framecrashed" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x66de } /* "@mozilla.org/network/protocol;1?name=moz-icon" */,
            ModuleID::Anonymous091,
          },
  
          {
            { 0x670c } /* "@mozilla.org/gecko-media-plugin-service;1" */,
            ModuleID::Anonymous440,
          },
  
          {
            { 0x6736 } /* "@mozilla.org/network/file-input-stream;1" */,
            ModuleID::Anonymous130,
          },
  
          {
            { 0x675f } /* "@mozilla.org/layout/contentserializer;1?mimetype=text/plain" */,
            ModuleID::Anonymous448,
          },
  
          {
            { 0x679b } /* "@mozilla.org/wifi/monitor;1" */,
            ModuleID::Anonymous194,
          },
  
          {
            { 0x67b7 } /* "@mozilla.org/uriloader/external-protocol-service;1" */,
            ModuleID::Anonymous030,
          },
  
          {
            { 0x67ea } /* "@mozilla.org/widget/taskbarprogress/gtk;1" */,
            ModuleID::Anonymous351,
          },
  
          {
            { 0x6814 } /* "@mozilla.org/addons/policy-service;1" */,
            ModuleID::ExtensionPolicy,
          },
  
          {
            { 0x6839 } /* "@mozilla.org/streamconv;1?from=multipart/byteranges&to=*\/\*" */,
            ModuleID::Anonymous179,
          },
  
          {
            { 0x6874 } /* "@mozilla.org/layout/contentserializer;1?mimetype=application/xml" */,
            ModuleID::Anonymous445,
          },
  
          {
            { 0x68b5 } /* "@mozilla.org/supports-PRBool;1" */,
            ModuleID::Anonymous389,
          },
  
          {
            { 0x68d4 } /* "@mozilla.org/network/protocol/about;1?what=about" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x6905 } /* "@mozilla.org/uriloader/dbus-handler-app;1" */,
            ModuleID::DBusHandlerApp,
          },
  
          {
            { 0x692f } /* "@mozilla.org/streamconv;1?from=multipart/mixed&to=*\/\*" */,
            ModuleID::Anonymous179,
          },
  
          {
            { 0x6965 } /* "@mozilla.org/io/multiplex-input-stream;1" */,
            ModuleID::Anonymous407,
          },
  
          {
            { 0x698e } /* "@mozilla.org/updates/update-processor;1" */,
            ModuleID::Anonymous253,
          },
  
          {
            { 0x69b6 } /* "@mozilla.org/security/contentsignatureverifier;1" */,
            ModuleID::Anonymous221,
          },
  
          {
            { 0x69e7 } /* "@mozilla.org/autocomplete/search;1?name=unifiedcomplete" */,
            ModuleID::Anonymous295,
          },
  
          {
            { 0x6a1f } /* "@mozilla.org/network/io-service;1" */,
            ModuleID::IO,
          },
  
          {
            { 0x6a41 } /* "@mozilla.org/network/protocol/about;1?what=devtools-toolbox" */,
            ModuleID::Anonymous022,
          },
  
          {
            { 0x6a7d } /* "@mozilla.org/services/settings;1" */,
            ModuleID::Anonymous235,
          },
  
          {
            { 0x6a9e } /* "@mozilla.org/network/dashboard;1" */,
            ModuleID::Anonymous125,
          },
  
          {
            { 0x6abf } /* "@mozilla.org/xpcom/debug;1" */,
            ModuleID::Anonymous368,
          },
  
          {
            { 0x6ada } /* "@mozilla.org/variant;1" */,
            ModuleID::Anonymous401,
          },
  
          {
            { 0x6af1 } /* "@mozilla.org/network/downloader;1" */,
            ModuleID::Anonymous128,
          },
  
          {
            { 0x6b13 } /* "@mozilla.org/network/incremental-stream-loader;1" */,
            ModuleID::Anonymous135,
          },
  
          {
            { 0x938 } /* "@mozilla.org/dom/payments/payment-abort-action-response;1" */,
            ModuleID::Anonymous062,
          },
  
          {
            { 0x6b44 } /* "@mozilla.org/network/protocol/about;1?what=debugging" */,
            ModuleID::Anonymous021,
          },
  
          {
            { 0x6b79 } /* "@mozilla.org/network/protocol/about;1?what=config" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x2125 } /* "@mozilla.org/alerts-service;1" */,
            ModuleID::Alerts,
          },
  
          {
            { 0x6bab } /* "@mozilla.org/login-manager/prompter;1" */,
            ModuleID::Anonymous284,
          },
  
          {
            { 0x6bd1 } /* "@mozilla.org/security/local-cert-service;1" */,
            ModuleID::Anonymous210,
          },
  
          {
            { 0x6bfc } /* "@mozilla.org/io-util;1" */,
            ModuleID::Anonymous376,
          },
  
          {
            { 0x6c13 } /* "@mozilla.org/profile/migrator;1?app=browser&type=chrome-beta" */,
            ModuleID::Anonymous010,
          },
  
          {
            { 0x6c50 } /* "@mozilla.org/scriptableinputstream;1" */,
            ModuleID::Anonymous409,
          },
  
          {
            { 0x6c75 } /* "@mozilla.org/browser/history;1" */,
            ModuleID::History,
          },
  
          {
            { 0x6c94 } /* "@mozilla.org/supports-PRInt16;1" */,
            ModuleID::Anonymous390,
          },
  
          {
            { 0x6cb4 } /* "@mozilla.org/scriptsecuritymanager;1" */,
            ModuleID::Anonymous465,
          },
  
          {
            { 0x6cd9 } /* "@mozilla.org/mozintl;1" */,
            ModuleID::Anonymous274,
          },
  
          {
            { 0x6cf0 } /* "@mozilla.org/enterprisepolicies;1" */,
            ModuleID::Anonymous268,
          },
  
          {
            { 0x6d12 } /* "@mozilla.org/network/protocol/about;1?what=rights" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x6d44 } /* "@mozilla.org/toolkit/crash-reporter;1" */,
            ModuleID::Anonymous341,
          },
  
          {
            { 0x6d6a } /* "@mozilla.org/network/protocol/about;1?what=sessionrestore" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x6da4 } /* "@mozilla.org/contentsecuritymanager;1" */,
            ModuleID::Anonymous425,
          },
  
          {
            { 0x6dca } /* "@mozilla.org/permissiondelegatehandler;1" */,
            ModuleID::Anonymous078,
          },
  
          {
            { 0x6df3 } /* "@mozilla.org/pipe;1" */,
            ModuleID::Anonymous408,
          },
  
          {
            { 0x6e07 } /* "@mozilla.org/login-manager;1" */,
            ModuleID::Anonymous277,
          },
  
          {
            { 0x6e24 } /* "@mozilla.org/toolkit/viaduct;1" */,
            ModuleID::Anonymous324,
          },
  
          {
            { 0x6e43 } /* "@mozilla.org/network/input-stream-channel;1" */,
            ModuleID::Anonymous136,
          },
  
          {
            { 0x6e6f } /* "@mozilla.org/gio-service;1" */,
            ModuleID::Anonymous337,
          },
  
          {
            { 0x6e8a } /* "@mozilla.org/scripterror;1" */,
            ModuleID::Anonymous464,
          },
  
          {
            { 0x6ea5 } /* "@mozilla.org/network/protocol/about;1?what=sync-log" */,
            ModuleID::Anonymous237,
          },
  
          {
            { 0x6ed9 } /* "@mozilla.org/passwordmanager/authpromptfactory;1" */,
            ModuleID::Anonymous278,
          },
  
          {
            { 0x6f0a } /* "@mozilla.org/intl/collation-factory;1" */,
            ModuleID::Anonymous095,
          },
  
          {
            { 0x6f30 } /* "@mozilla.org/uriloader/content-handler;1?type=text/plain" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x6f69 } /* "@mozilla.org/embeddor.implemented/web-protocol-handler-registrar;1" */,
            ModuleID::Anonymous014,
          },
  
          {
            { 0x6fac } /* "@mozilla.org/binaryinputstream;1" */,
            ModuleID::Anonymous371,
          },
  
          {
            { 0x6fcd } /* "@mozilla.org/content/html-menu-builder;1" */,
            ModuleID::Anonymous039,
          },
  
          {
            { 0x6ff6 } /* "@mozilla.org/network/protocol;1?name=moz-safe-about" */,
            ModuleID::Anonymous155,
          },
  
          {
            { 0x702a } /* "@mozilla.org/plugin/host;1" */,
            ModuleID::Anonymous068,
          },
  
          {
            { 0xd0d } /* "@mozilla.org/toolkit/shutdown-terminator-telemetry;1" */,
            ModuleID::Anonymous314,
          },
  
          {
            { 0x7045 } /* "@mozilla.org/file/directory_service;1" */,
            ModuleID::Directory,
          },
  
          {
            { 0x706b } /* "@mozilla.org/mozintlhelper;1" */,
            ModuleID::Anonymous273,
          },
  
          {
            { 0x8eb } /* "@mozilla.org/dom/payments/general-response-data;1" */,
            ModuleID::Anonymous061,
          },
  
          {
            { 0x80a } /* "@mozilla.org/dom/payments/basiccard-change-details;1" */,
            ModuleID::Anonymous058,
          },
  
          {
            { 0x7088 } /* "@mozilla.org/network/protocol/about;1?what=cache-entry" */,
            ModuleID::Anonymous146,
          },
  
          {
            { 0x70bf } /* "@mozilla.org/extensions/storage/sync;1" */,
            ModuleID::Anonymous270,
          },
  
          {
            { 0x70e6 } /* "@mozilla.org/network/stream-listener-tee;1" */,
            ModuleID::Anonymous167,
          },
  
          {
            { 0x7111 } /* "@mozilla.org/categorymanager;1" */,
            ModuleID::Anonymous403,
          },
  
          {
            { 0x855 } /* "@mozilla.org/dom/payments/basiccard-response-data;1" */,
            ModuleID::Anonymous059,
          },
  
          {
            { 0x7130 } /* "@mozilla.org/dom/peerconnection;1" */,
            ModuleID::Anonymous043,
          },
  
          {
            { 0x7152 } /* "@mozilla.org/network/safe-file-output-stream;1" */,
            ModuleID::Anonymous160,
          },
  
          {
            { 0x7181 } /* "@mozilla.org/network/protocol/about;1?what=newinstall" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x71b7 } /* "@mozilla.org/network/protocol/about;1?what=neterror" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x71eb } /* "@mozilla.org/network/predictor;1" */,
            ModuleID::Anonymous142,
          },
  
          {
            { 0x720c } /* "@mozilla.org/network/protocol;1?name=wss" */,
            ModuleID::Anonymous159,
          },
  
          {
            { 0x7235 } /* "@mozilla.org/network/protocol/about;1?what=httpsonlyerror" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x726f } /* "@mozilla.org/sandbox/syscall-reporter;1" */,
            ModuleID::Anonymous231,
          },
  
          {
            { 0x7297 } /* "@mozilla.org/updates/update-service;1" */,
            ModuleID::Anonymous333,
          },
  
          {
            { 0x72bd } /* "@mozilla.org/reputationservice/login-reputation-service;1" */,
            ModuleID::LoginReputation,
          },
  
          {
            { 0x72f7 } /* "@mozilla.org/network/captive-portal-service;1" */,
            ModuleID::Anonymous124,
          },
  
          {
            { 0x7325 } /* "@mozilla.org/network/protocol/about;1?what=privatebrowsing" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x7360 } /* "@mozilla.org/text-input-processor;1" */,
            ModuleID::Anonymous468,
          },
  
          {
            { 0x7384 } /* "@mozilla.org/network/protocol/about;1?what=crashparent" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x73bb } /* "@mozilla.org/uuid-generator;1" */,
            ModuleID::UUIDGenerator,
          },
  
          {
            { 0x73d9 } /* "@mozilla.org/satchel/inputlist-autocomplete;1" */,
            ModuleID::Anonymous306,
          },
  
          {
            { 0x20e2 } /* "@mozilla.org/system-info;1" */,
            ModuleID::Anonymous399,
          },
  
          {
            { 0x7407 } /* "@mozilla.org/network/protocol/about;1?what=welcomeback" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x743e } /* "@mozilla.org/security/pkcs11moduledb;1" */,
            ModuleID::Anonymous213,
          },
  
          {
            { 0x7465 } /* "@mozilla.org/network/protocol/about;1?what=performance" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x749c } /* "@mozilla.org/tracking-url-decoration-service;1" */,
            ModuleID::Anonymous242,
          },
  
          {
            { 0x74cb } /* "@mozilla.org/uriloader/content-handler;1?type=image/jpg" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x7503 } /* "@mozilla.org/url-classifier/prefixset;1" */,
            ModuleID::UrlClassifierPrefixSet,
          },
  
          {
            { 0x752b } /* "@mozilla.org/network/protocol/about;1?what=restartrequired" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x3ba } /* "@mozilla.org/commandlinehandler/general-startup;1?type=recording" */,
            ModuleID::Anonymous103,
          },
  
          {
            { 0x7566 } /* "@mozilla.org/webspeech/service;1?name=fake" */,
            ModuleID::Anonymous473,
          },
  
          {
            { 0x7591 } /* "@mozilla.org/browser/aboutnewtab-service;1" */,
            ModuleID::Anonymous012,
          },
  
          {
            { 0x75bc } /* "@mozilla.org/image/encoder;2?type=image/png" */,
            ModuleID::Anonymous090,
          },
  
          {
            { 0x75e8 } /* "@mozilla.org/systemprincipal;1" */,
            ModuleID::Anonymous467,
          },
  
          {
            { 0x7607 } /* "@mozilla.org/nss_errors_service;1" */,
            ModuleID::Anonymous208,
          },
  
          {
            { 0x7629 } /* "@mozilla.org/storage/service;1" */,
            ModuleID::Anonymous239,
          },
  
          {
            { 0x7648 } /* "@mozilla.org/cookiemanager;1" */,
            ModuleID::Anonymous191,
          },
  
          {
            { 0x7665 } /* "@mozilla.org/uriloader/content-handler;1?type=image/vnd.microsoft.icon" */,
            ModuleID::Anonymous002,
          },
  
          {
            { 0x76ac } /* "@mozilla.org/addons/addon-manager-startup;1" */,
            ModuleID::Anonymous249,
          },
  
          {
            { 0x76d8 } /* "@mozilla.org/network/protocol/about;1?what=profiles" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x770c } /* "@mozilla.org/network/buffered-input-stream;1" */,
            ModuleID::Anonymous121,
          },
  
          {
            { 0x7739 } /* "@mozilla.org/network/protocol/about;1?what=profiling" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x776e } /* "@mozilla.org/dom/peerconnectionmanager;1" */,
            ModuleID::Anonymous048,
          },
  
          {
            { 0x7797 } /* "@mozilla.org/network/protocol;1?name=moz-anno" */,
            ModuleID::Anonymous287,
          },
  
          {
            { 0x77c5 } /* "@mozilla.org/extensions/blocklist;1" */,
            ModuleID::Anonymous327,
          },
  
          {
            { 0x77e9 } /* "@mozilla.org/content-pref/service;1" */,
            ModuleID::Anonymous261,
          },
  
          {
            { 0x780d } /* "@mozilla.org/network/native-dns-override;1" */,
            ModuleID::Anonymous127,
          },
  
          {
            { 0x7838 } /* "@mozilla.org/psm;1" */,
            ModuleID::Anonymous207,
          },
  
          {
            { 0x784b } /* "@mozilla.org/network/protocol/about;1?what=cache" */,
            ModuleID::Anonymous145,
          },
  
          {
            { 0x787c } /* "@mozilla.org/network/protocol;1?name=moz-extension" */,
            ModuleID::Anonymous153,
          },
  
          {
            { 0x78af } /* "@mozilla.org/embedding/browser/nsWebBrowserPersist;1" */,
            ModuleID::Anonymous343,
          },
  
          {
            { 0x78e4 } /* "@mozilla.org/network/protocol/about;1?what=downloads" */,
            ModuleID::Anonymous000,
          },
  
          {
            { 0x7919 } /* "@mozilla.org/image/encoder;2?type=image/vnd.microsoft.icon" */,
            ModuleID::Anonymous088,
          },
  
          {
            { 0x7954 } /* "@mozilla.org/network/protocol/about;1?what=networking" */,
            ModuleID::Anonymous024,
          },
  
          {
            { 0x798a } /* "@mozilla.org/docshell/uri-fixup;1" */,
            ModuleID::URIFixup,
          },
  
          {
            { 0x79ac } /* "@mozilla.org/spellchecker/personaldictionary;1" */,
            ModuleID::Anonymous081,
          },
  
          {
            { 0x79db } /* "@mozilla.org/image/request;1" */,
            ModuleID::Anonymous085,
          },
  
          {
            { 0x79f8 } /* "@mozilla.org/network/load-group;1" */,
            ModuleID::Anonymous138,
          }
};



const ContractEntry*
LookupContractID(const nsACString& aKey)
{
  static const uint16_t BASES[] = {
       2,   1,   2,   0,   0,   3,   1,   1,   6,   0,   0,   1,   0,   1,   3,   0,
       5,   3,   0,   0,   0,   0,   4,   1,   2,   1,   5,   1,   6,   0,   2,   1,
       0,   7,   1,   7,   0,   0,   0,   1,   0,   0,   0,   0,   1,   1,   3,   3,
       0,   2,   3,   1,   1,   8,   0,   2,   1,   2,   4,   3,   0,   0,   2,   2,
       3,   1,   1,   1,   3,   0,   1,   0,   0,   0,   2,   1,   0,   0,   3,   2,
       0,   1,   4,   1,   0,   0,   1,   6,   1,   0,   3,   0,   0,   0,   2,   0,
       5,   2,   7,   1,   0,   0,   2,   2,   1,   1,   5,   1,   1,   2,   1,   5,
       6,   3,   1,   1,   0,   1,   1,   0,   0,   0,   2,   0,   1,   0,   1,   2,
       2,   1,   3,   0,   1,   0,   9,   0,   1,   0,   0,   0,   2,   6,   1,   0,
       2,  10,   1,   0,   4,   2,   3,   0,   4,   8,   2,   0,   5,   0,   3,   1,
       4,   1,   1,  10,   3,   0,   1,   5,   0,   8,   1,   1,   0,   2,   2,   1,
       7,   2,  13,   6,   9,   2,   2,   4,   4,   0,   0,   2,   8,   2,  10,   0,
       0,   2,   3,   0,   2,   3,   2,   1,   1,   0,   7,   3,   3,   0,   5,   7,
       1,  13,   1,   8,   2,   6,   0,  10,   0,   0,   3,   1,   4,   0,   0,   7,
       1,   0,   1,   7,   2,   1,   5,   0,   9,   7,   1,   0,  12,   1,   2,   1,
       4,   1,   1,   2,   6,   2,   0,  10,   0,   1,   0,   2,   1,   0,   0,   8,
       0,   0,   1,   0,   3,   0,   0,   1,   0,   1,   0,   8,   0,   3,   0,   1,
      10,   2,   0,   0,   4,  17,   2,  19,   0,   7,   5,   3,   5,   2,   0,   6,
       0,   5,   0,  12,   4,   9,   1,   0,   1,   2,   1,   8,   0,   1,   2,   2,
       4,   0,   0,   1,   0,   1,   0,   1,   0,   7,   0,   0,   7,   0,   9,   9,
       1,   0,   1,   0,   3,   6,   1,   1,   0,   8,  12,   0,   0,   3,   3,   0,
       6,   2,   3,   1,   5,   8,   8,   0,   2,   6,   0,   0,   5,  17,  12,   0,
       4,   0,  10,   0,   0,  16,   5,   1,   0,   0,   3,   6,   0,   4,   0,   1,
       5,   4,   2,  23,   2,   0,   0,   4,   3,   2,   1,   0,   5,   1,   7,   2,
       4,   0,   8,   8,   5,   4,   0,   1,   5,   0,   9,   0,   0,  11,  16,   0,
       0,   0,   8,   0,   4,  14,   5,   5,   4,  10,   9,   3,  11,  42,   2,   6,
       0,   9,  23,   0,   0,   0,   4,  17,   7,   7,  29,   2,   1,   0,   3,   1,
       0,   0,   2,  12,   8,   0,  10,   2,   0,   1,   3,   7,  15,   0,   0,  11,
       2,  58,  23,   5,   0,   1,  40,   1,  11,   6,  10,   0,   0,  19,   2,   0,
       8,   0,   0,   5,  10,   2,   0,   0,  11,  19,   0,   9,   0,   5,   9,   0,
      12,   3,  40,   0,  34,  18,   0,   4,  54, 162,  54,   2,   1,  13,  32,   3,
       0,  14,   0,  10,  98, 107, 270,   0,  13,  62, 323,   7,   1,1424,   0,   0,
  };
  

  const char* bytes = aKey.BeginReading();
  size_t length = aKey.Length();
  auto& entry = mozilla::perfecthash::Lookup(bytes, length, BASES,
                                             gContractEntries);
  return entry.Matches(aKey) ? &entry : nullptr;
}

const JSServiceEntry gJSServices[] = {
  
          {
            { 0x7a1a } /* "profiler" */,
            ModuleID::Anonymous347,
            { 29 },
            1
          },
  
          {
            { 0x7a23 } /* "uriFixup" */,
            ModuleID::URIFixup,
            { 0 },
            0
          },
  
          {
            { 0x7a2c } /* "prompt" */,
            ModuleID::Anonymous302,
            { 21 },
            1
          },
  
          {
            { 0x7a33 } /* "textToSubURI" */,
            ModuleID::Anonymous099,
            { 4 },
            1
          },
  
          {
            { 0x7a40 } /* "cookies" */,
            ModuleID::Anonymous191,
            { 14 },
            1
          },
  
          {
            { 0x7a48 } /* "io" */,
            ModuleID::IO,
            { 11 },
            3
          },
  
          {
            { 0x7a4b } /* "tm" */,
            ModuleID::Anonymous400,
            { 34 },
            1
          },
  
          {
            { 0x7a4e } /* "prefs" */,
            ModuleID::Anonymous111,
            { 6 },
            2
          },
  
          {
            { 0x7a54 } /* "strings" */,
            ModuleID::StringBundle,
            { 2 },
            1
          },
  
          {
            { 0x7a5c } /* "els" */,
            ModuleID::Anonymous437,
            { 46 },
            1
          },
  
          {
            { 0x7a60 } /* "storage" */,
            ModuleID::Anonymous239,
            { 15 },
            1
          },
  
          {
            { 0x7a68 } /* "eTLD" */,
            ModuleID::Anonymous129,
            { 10 },
            1
          },
  
          {
            { 0x7a6d } /* "perms" */,
            ModuleID::PermissionManager,
            { 1 },
            1
          },
  
          {
            { 0x7a73 } /* "catMan" */,
            ModuleID::Anonymous403,
            { 36 },
            1
          },
  
          {
            { 0x7a7a } /* "scriptSecurityManager" */,
            ModuleID::Anonymous465,
            { 48 },
            1
          },
  
          {
            { 0x7a90 } /* "focus" */,
            ModuleID::Anonymous439,
            { 47 },
            1
          },
  
          {
            { 0x7a96 } /* "cpmm" */,
            ModuleID::Anonymous421,
            { 42 },
            0
          },
  
          {
            { 0x7a9b } /* "dirsvc" */,
            ModuleID::Directory,
            { 38 },
            2
          },
  
          {
            { 0x7aa2 } /* "blocklist" */,
            ModuleID::Anonymous327,
            { 26 },
            0
          },
  
          {
            { 0x7aac } /* "domStorageManager" */,
            ModuleID::Anonymous432,
            { 43 },
            2
          },
  
          {
            { 0x7abe } /* "locale" */,
            ModuleID::Anonymous093,
            { 3 },
            1
          },
  
          {
            { 0x7ac5 } /* "startup" */,
            ModuleID::AppStartup,
            { 16 },
            1
          },
  
          {
            { 0x7acd } /* "console" */,
            ModuleID::Anonymous374,
            { 31 },
            1
          },
  
          {
            { 0x7ad5 } /* "urlFormatter" */,
            ModuleID::Anonymous321,
            { 24 },
            1
          },
  
          {
            { 0x7ae2 } /* "loadContextInfo" */,
            ModuleID::Anonymous114,
            { 8 },
            1
          },
  
          {
            { 0x7af2 } /* "scriptloader" */,
            ModuleID::Anonymous102,
            { 5 },
            1
          },
  
          {
            { 0x7aff } /* "policies" */,
            ModuleID::Anonymous268,
            { 18 },
            1
          },
  
          {
            { 0x7b08 } /* "mm" */,
            ModuleID::Anonymous442,
            { 48 },
            0
          },
  
          {
            { 0x7b0b } /* "qms" */,
            ModuleID::Anonymous434,
            { 45 },
            1
          },
  
          {
            { 0x7b0f } /* "wm" */,
            ModuleID::Anonymous414,
            { 41 },
            1
          },
  
          {
            { 0x7b12 } /* "vc" */,
            ModuleID::Anonymous402,
            { 35 },
            1
          },
  
          {
            { 0x7b15 } /* "logins" */,
            ModuleID::Anonymous277,
            { 20 },
            1
          },
  
          {
            { 0x7b1c } /* "ppmm" */,
            ModuleID::Anonymous456,
            { 48 },
            0
          },
  
          {
            { 0x7b21 } /* "telemetry" */,
            ModuleID::Anonymous313,
            { 23 },
            1
          },
  
          {
            { 0x7b2b } /* "appShell" */,
            ModuleID::Anonymous413,
            { 40 },
            1
          },
  
          {
            { 0x7b34 } /* "crashmanager" */,
            ModuleID::Anonymous263,
            { 18 },
            0
          },
  
          {
            { 0x7b41 } /* "sysinfo" */,
            ModuleID::Anonymous399,
            { 32 },
            2
          },
  
          {
            { 0x7b49 } /* "cache2" */,
            ModuleID::CacheStorage,
            { 9 },
            1
          },
  
          {
            { 0x7b50 } /* "xulStore" */,
            ModuleID::Anonymous325,
            { 25 },
            1
          },
  
          {
            { 0x7b59 } /* "droppedLinkHandler" */,
            ModuleID::Anonymous031,
            { 0 },
            1
          },
  
          {
            { 0x7b6c } /* "DOMRequest" */,
            ModuleID::Anonymous431,
            { 42 },
            1
          },
  
          {
            { 0x7b77 } /* "appinfo" */,
            ModuleID::Anonymous341,
            { 26 },
            2
          },
  
          {
            { 0x7b7f } /* "intl" */,
            ModuleID::Anonymous274,
            { 19 },
            1
          },
  
          {
            { 0x7b84 } /* "clearData" */,
            ModuleID::Anonymous259,
            { 17 },
            1
          },
  
          {
            { 0x7b8e } /* "clipboard" */,
            ModuleID::Anonymous359,
            { 30 },
            1
          },
  
          {
            { 0x7b98 } /* "search" */,
            ModuleID::Anonymous308,
            { 22 },
            1
          },
  
          {
            { 0x7b9f } /* "obs" */,
            ModuleID::Anonymous406,
            { 37 },
            1
          },
  
          {
            { 0x7ba3 } /* "ww" */,
            ModuleID::Anonymous344,
            { 28 },
            1
          }
};



const JSServiceEntry*
LookupJSService(const nsACString& aKey)
{
  static const uint8_t BASES[] = {
       0,   0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   1,
       2,   0,   0,   1,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   1,   0,   0,   0,   1,   0,   1,   0,   0,   0,   0,   0,   0,   0,   1,
       0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   2,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   3,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   2,   0,   1,   0,   0,   2,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   3,   0,   1,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   2,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   4,
       0,   0,   0,   0,   5,   0,   0,   0,   0,   0,   1,   0,   0,   1,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   4,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   4,   0,   0,   0,   0,   1,   0,   1,   0,   0,   0,
       0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   1,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  13,
       0,   0,   0,   0,   0,   5,   0,   2,   0,   0,   0,   0,   0,   0,   0,   0,
      10,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   2,   0,   0,   0,   0,
       0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   5,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   4,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   2,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,  10,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
       0,   0,   0,   0,   0,   0,   0,  32,   0,   0,   0,   0,   0,   0,   0,   0,
  };
  

  const char* bytes = aKey.BeginReading();
  size_t length = aKey.Length();
  auto& entry = mozilla::perfecthash::Lookup(bytes, length, BASES,
                                             gJSServices);
  return entry.Name() == aKey ? &entry : nullptr;
}

static inline bool CalledInit(size_t aIdx) {
  return GetBit(gInitCalled, aIdx);
}

static nsresult CallInitFunc(size_t aIdx) {
  if (CalledInit(aIdx)) {
    return NS_OK;
  }

  nsresult rv = NS_OK;
  switch (aIdx) {
    case 0:
      mozilla::InitDocShellModule();
      break;
    case 1:
      mozilla::image::EnsureModuleInitialized();
      break;
    case 2:
      /* empty */
      break;
    case 3:
      nsNetStartup();
      break;
    case 4:
      nsParserInitialize();
      break;
    case 5:
      /* empty */
      break;
    case 6:
      nsWidgetGtk2ModuleCtor();
      break;
    case 7:
      /* empty */
      break;
  }

  SetBit(gInitCalled, aIdx);

  MOZ_ASSERT(NS_SUCCEEDED(rv));
  return rv;
}

static void CallUnloadFuncs() {
  if (CalledInit(0)) {
    mozilla::UnloadDocShellModule();
  }
  if (CalledInit(2)) {
    mozilla::UnloadPrefsModule();
  }
  if (CalledInit(3)) {
    nsNetShutdown();
  }
  if (CalledInit(4)) {
    nsHTMLTags::ReleaseTable();
  }
  if (CalledInit(5)) {
    mozilla::Telemetry::ShutdownTelemetry();
  }
  if (CalledInit(6)) {
    nsWidgetGtk2ModuleDtor();
  }
  if (CalledInit(7)) {
    nsLayoutModuleDtor();
  }
}

static nsresult CreateInstanceImpl(ModuleID aID, nsISupports* aOuter,
                                   const nsIID& aIID, void** aResult) {
  if (aOuter) {
    return NS_ERROR_NO_AGGREGATION;
  }

  // The full set of constructors for all static modules.
  // This switch statement will be compiled to a relative address jump table
  // with no runtime relocations and a single indirect jump.
  switch (aID) {
    case ModuleID::Anonymous166: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsStandardURL::Mutator> inst = new mozilla::net::nsStandardURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous157: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsViewSourceHandler> inst = new mozilla::net::nsViewSourceHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous115: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsApplicationCacheService> inst = new nsApplicationCacheService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous036: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ConsoleAPIStorage.jsm"),
                                    "ConsoleAPIStorageService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous039: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/HTMLMenuBuilder.jsm"),
                                    "HTMLMenuBuilder",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous046: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCIceCandidate",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous159: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ mozilla::net::WebSocketSSLChannelConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous093: {
      RefPtr<mozilla::intl::LocaleService> inst = mozilla::intl::LocaleService::GetInstanceAddRefed();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::intl::LocaleService::GetInstanceAddRefed())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::intl::LocaleService::GetInstanceAddRefed())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::intl::LocaleService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous360: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsClipboardHelper> inst = new nsClipboardHelper();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous278: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginManagerAuthPrompter.jsm"),
                                    "LoginManagerAuthPromptFactory",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous108: {
      RefPtr<nsZipReaderCache> inst = new nsZipReaderCache();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous195: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsNetworkInfoService> inst = new mozilla::net::nsNetworkInfoService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous221: {
      return /* legacy */ mozilla::psm::NSSConstructor<ContentSignatureVerifier>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous192: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsISupports> inst = mozilla::net::GetSFVService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::GetSFVService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::GetSFVService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsISupports, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous103: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/RecordingCmdLine.jsm"),
                                    "RecordingCmdLineHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous070: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PresentationDataChannelSessionTransport.jsm"),
                                    "PresentationTransport",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous284: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginManagerPrompter.jsm"),
                                    "LoginManagerPrompter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous273: {
      RefPtr<mozilla::MozIntlHelper> inst = new mozilla::MozIntlHelper();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous047: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCSessionDescription",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous320: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UrlClassifierHashCompleter.jsm"),
                                    "HashCompleter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous289: {
      RefPtr<nsNavBookmarks> inst = nsNavBookmarks::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsNavBookmarks::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsNavBookmarks::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsNavBookmarks, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous029: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsWebNavigationInfo> inst = new nsWebNavigationInfo();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous315: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PageThumbsStorageService.jsm"),
                                    "PageThumbsStorageService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous262: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/CrashService.jsm"),
                                    "CrashService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous419: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::BlobURL::Mutator> inst = new mozilla::dom::BlobURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::IO: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsIOService> inst = mozilla::net::nsIOService::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::nsIOService::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::nsIOService::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::net::nsIOService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous059: {
      RefPtr<mozilla::dom::BasicCardResponseData> inst = new mozilla::dom::BasicCardResponseData();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous347: {
      RefPtr<nsProfiler> inst = new nsProfiler();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous074: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/Push.jsm"),
                                    "Push",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::OfflineCacheUpdate: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsOfflineCacheUpdateService> inst = nsOfflineCacheUpdateService::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsOfflineCacheUpdateService::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsOfflineCacheUpdateService::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsOfflineCacheUpdateService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous408: {
      return /* legacy */ nsPipeConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous048: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "GlobalPCList",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous294: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PlacesDBUtils.jsm"),
                                    "PlacesDBUtilsIdleMaintenance",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::URILoader: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsURILoader> inst = new nsURILoader();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous172: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsStdURLParser> inst = new nsStdURLParser();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous438: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<ExpandedPrincipal> inst = new ExpandedPrincipal();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous322: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/SimpleServices.jsm"),
                                    "AddonLocalizationConverter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous006: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/ProfileMigrator.jsm"),
                                    "ProfileMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous163: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSimpleStreamListener> inst = new mozilla::net::nsSimpleStreamListener();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous254: {
      RefPtr<nsUpdateSyncManager> inst = nsUpdateSyncManager::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsUpdateSyncManager::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsUpdateSyncManager::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsUpdateSyncManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous199: {
      RefPtr<nsGIOProtocolHandler> inst = nsGIOProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsGIOProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsGIOProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsGIOProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous359: {
      MOZ_TRY(CallInitFunc(6));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIClipboard>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous052: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCRtpReceiver",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous067: {
      RefPtr<mozilla::dom::PaymentShowActionResponse> inst = new mozilla::dom::PaymentShowActionResponse();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous402: {
      RefPtr<nsVersionComparatorImpl> inst = new nsVersionComparatorImpl();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous217: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsCryptoHash>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous014: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/WebProtocolHandlerRegistrar.jsm"),
                                    "WebProtocolHandlerRegistrar",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous165: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::DefaultURI::Mutator> inst = new mozilla::net::DefaultURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous173: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsNoAuthURLParser> inst = new nsNoAuthURLParser();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous131: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsFileOutputStream::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous343: {
      RefPtr<nsWebBrowserPersist> inst = new nsWebBrowserPersist();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous445: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateXMLContentSerializer(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous118: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::BackgroundFileSaverOutputStream> inst = new mozilla::net::BackgroundFileSaverOutputStream();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous319: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UrlClassifierListManager.jsm"),
                                    "RegistrationData",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous183: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSimpleNestedURI::Mutator> inst = new mozilla::net::nsSimpleNestedURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous337: {
      RefPtr<nsGIOService> inst = new nsGIOService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous434: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::quota::QuotaManagerService> inst = mozilla::dom::quota::QuotaManagerService::FactoryCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::quota::QuotaManagerService::FactoryCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::quota::QuotaManagerService::FactoryCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::quota::QuotaManagerService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous325: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/XULStore.jsm"),
                                    "XULStore",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous096: {
      RefPtr<nsConverterInputStream> inst = new nsConverterInputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous042: {
      RefPtr<nsStunUDPSocketFilterHandler> inst = new nsStunUDPSocketFilterHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous466: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::StorageActivityService> inst = mozilla::dom::StorageActivityService::GetOrCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::StorageActivityService::GetOrCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::StorageActivityService::GetOrCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::StorageActivityService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous317: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TooltipTextProvider.jsm"),
                                    "TooltipTextProvider",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous293: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PlacesExpiration.jsm"),
                                    "nsPlacesExpiration",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::ThirdPartyUtil: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<ThirdPartyUtil> inst = new ThirdPartyUtil();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous239: {
      RefPtr<mozilla::storage::Service> inst = mozilla::storage::Service::getSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::storage::Service::getSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::storage::Service::getSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::storage::Service, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous051: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCRtpSender",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous229: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsCertTree>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous019: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/AboutDevToolsRegistration.jsm"),
                                    "AboutDevtools",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::SocketTransport: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSocketTransportService> inst = new mozilla::net::nsSocketTransportService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous309: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/SearchSuggestions.jsm"),
                                    "SearchSuggestAutoComplete",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Directory: {
      return /* legacy */ nsDirectoryService::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous263: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/CrashManager.jsm"),
                                    "getCrashManager",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous298: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/MainProcessSingleton.jsm"),
                                    "MainProcessSingleton",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous206: {
      RefPtr<nsNSSDialogs> inst = new nsNSSDialogs();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous060: {
      RefPtr<mozilla::dom::GeneralMethodChangeDetails> inst = new mozilla::dom::GeneralMethodChangeDetails();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous233: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIIdentityCryptoService>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous458: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::PresentationDeviceManager> inst = new mozilla::dom::PresentationDeviceManager();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::UrlClassifierDB: {
      return /* legacy */ nsUrlClassifierDBServiceConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous149: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsFileProtocolHandler> inst = new nsFileProtocolHandler();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous100: {
      RefPtr<mozilla::ipc::ForkServerLauncher> inst = mozilla::ipc::ForkServerLauncher::Create();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::ipc::ForkServerLauncher::Create())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::ipc::ForkServerLauncher::Create())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::ipc::ForkServerLauncher, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::GfxInfo: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<mozilla::widget::GfxInfo> inst = new mozilla::widget::GfxInfo();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous367: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsPrinterListCUPS> inst = new nsPrinterListCUPS();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous350: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<mozilla::widget::ScreenManager> inst = mozilla::widget::ScreenManager::GetAddRefedSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::widget::ScreenManager::GetAddRefedSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::widget::ScreenManager::GetAddRefedSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::widget::ScreenManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous440: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::gmp::GeckoMediaPluginService> inst = mozilla::gmp::GeckoMediaPluginService::GetGeckoMediaPluginService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::gmp::GeckoMediaPluginService::GetGeckoMediaPluginService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::gmp::GeckoMediaPluginService::GetGeckoMediaPluginService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::gmp::GeckoMediaPluginService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous456: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateParentMessageManager(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous227: {
      return /* legacy */ mozilla::psm::NSSConstructor<OSReauthenticator>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous474: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIAccessibilityService>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous225: {
      RefPtr<nsSiteSecurityService> inst = new nsSiteSecurityService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous448: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreatePlainTextSerializer(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous439: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIFocusManager>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::UrlClassifierUtils: {
      RefPtr<nsUrlClassifierUtils> inst = nsUrlClassifierUtils::GetXPCOMSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsUrlClassifierUtils::GetXPCOMSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsUrlClassifierUtils::GetXPCOMSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsUrlClassifierUtils, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::HttpActivityDistributor: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsHttpActivityDistributor> inst = new mozilla::net::nsHttpActivityDistributor();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous174: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsAuthURLParser> inst = new nsAuthURLParser();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous084: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<mozilla::image::ImageBlocker> inst = new mozilla::image::ImageBlocker();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous255: {
      RefPtr<nsParentalControlsService> inst = new nsParentalControlsService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous427: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<CSPService> inst = new CSPService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous316: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UpdateTimerManager.jsm"),
                                    "TimerManager",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous158: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ mozilla::net::WebSocketChannelConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous175: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewStreamConvServiceFactory(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous249: {
      RefPtr<mozilla::AddonManagerStartup> inst = mozilla::AddonManagerStartup::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::AddonManagerStartup::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::AddonManagerStartup::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::AddonManagerStartup, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous076: {
      RefPtr<nsIndexedDBProtocolHandler> inst = new nsIndexedDBProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous420: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::BlobURL::Mutator> inst = new mozilla::dom::BlobURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous423: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIDocumentLoaderFactory>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous424: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsStyleSheetService> inst = new nsStyleSheetService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous079: {
      RefPtr<nsReadConfig> inst = new nsReadConfig();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous197: {
      RefPtr<mozilla::net::nsDNSServiceInfo> inst = new mozilla::net::nsDNSServiceInfo();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous069: {
      RefPtr<mozilla::dom::MockedSocketTransport> inst = new mozilla::dom::MockedSocketTransport();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous235: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://services-settings/RemoteSettingsComponents.jsm"),
                                    "RemoteSettingsTimer",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous021: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/AboutDebuggingRegistration.jsm"),
                                    "AboutDebugging",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous431: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::DOMRequestService> inst = mozilla::dom::DOMRequestService::FactoryCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::DOMRequestService::FactoryCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::DOMRequestService::FactoryCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::DOMRequestService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous390: {
      RefPtr<nsSupportsPRInt16> inst = new nsSupportsPRInt16();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous446: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateXHTMLContentSerializer(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous128: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsDownloader> inst = new nsDownloader();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous346: {
      RefPtr<nsDialogParamBlock> inst = new nsDialogParamBlock();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous011: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/ChromeProfileMigrator.jsm"),
                                    "ChromeDevMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous247: {
      RefPtr<nsAutoCompleteSimpleResult> inst = new nsAutoCompleteSimpleResult();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous271: {
      RefPtr<mozilla::FOG> inst = mozilla::FOG::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::FOG::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::FOG::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::FOG, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous388: {
      RefPtr<nsSupportsInterfacePointer> inst = new nsSupportsInterfacePointer();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous381: {
      RefPtr<nsScriptableBase64Encoder> inst = new nsScriptableBase64Encoder();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous429: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsDeviceSensors> inst = new nsDeviceSensors();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous156: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsResProtocolHandler> inst = nsResProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsResProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsResProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsResProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous361: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsDragService> inst = nsDragService::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsDragService::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsDragService::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsDragService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous335: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UpdateService.jsm"),
                                    "Checker",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous303: {
      RefPtr<mozilla::reflect::Module> inst = new mozilla::reflect::Module();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous230: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozISandboxSettings>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous295: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UnifiedComplete.jsm"),
                                    "UnifiedComplete",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous187: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsStandardURL::Mutator> inst = new mozilla::net::nsStandardURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous058: {
      RefPtr<mozilla::dom::BasicCardMethodChangeDetails> inst = new mozilla::dom::BasicCardMethodChangeDetails();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous265: {
      RefPtr<mozilla::ctypes::Module> inst = new mozilla::ctypes::Module();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous003: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/BrowserContentHandler.jsm"),
                                    "nsDefaultCommandLineHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous224: {
      return /* legacy */ mozilla::psm::NSSConstructor<mozilla::psm::TransportSecurityInfo>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous362: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsUserIdleService> inst = nsUserIdleServiceGTK::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsUserIdleServiceGTK::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsUserIdleServiceGTK::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsUserIdleService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous198: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/DNSServiceDiscovery.jsm"),
                                    "nsDNSServiceDiscovery",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous141: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsINetworkConnectivityService> inst = mozilla::net::NetworkConnectivityService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::NetworkConnectivityService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::NetworkConnectivityService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsINetworkConnectivityService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous399: {
      RefPtr<nsSystemInfo> inst = new nsSystemInfo();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous413: {
      RefPtr<nsAppShellService> inst = new nsAppShellService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous400: {
      return /* legacy */ nsThreadManagerGetSingleton(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous441: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::Geolocation> inst = mozilla::dom::Geolocation::NonWindowSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::Geolocation::NonWindowSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::Geolocation::NonWindowSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::Geolocation, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous208: {
      RefPtr<mozilla::psm::NSSErrorsService> inst = new mozilla::psm::NSSErrorsService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous380: {
      RefPtr<nsProcess> inst = new nsProcess();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous391: {
      RefPtr<nsSupportsPRInt32> inst = new nsSupportsPRInt32();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous442: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateGlobalMessageManager(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous234: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/FxAccountsPush.jsm"),
                                    "FxAccountsPushService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous160: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsSafeFileOutputStream> inst = new nsSafeFileOutputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::FindService: {
      RefPtr<nsFindService> inst = new nsFindService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous087: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<nsBMPEncoder> inst = new nsBMPEncoder();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous308: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/SearchService.jsm"),
                                    "SearchService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous430: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsStructuredCloneContainer> inst = new nsStructuredCloneContainer();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous216: {
      RefPtr<nsClientAuthRememberService> inst = new nsClientAuthRememberService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous184: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSimpleNestedURI::Mutator> inst = new mozilla::net::nsSimpleNestedURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous032: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ProcessSelector.jsm"),
                                    "RandomSelector",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous193: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsICookieJarSettings> inst = mozilla::net::CookieJarSettings::Create();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::CookieJarSettings::Create())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::CookieJarSettings::Create())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsICookieJarSettings, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous007: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/ChromeProfileMigrator.jsm"),
                                    "ChromeProfileMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous196: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsNetworkLinkService> inst = new nsNetworkLinkService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous130: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsFileInputStream::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous267: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/DownloadLegacy.jsm"),
                                    "DownloadLegacyTransfer",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous138: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsLoadGroupConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous016: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://devtools/client/jsonview/Sniffer.jsm"),
                                    "Sniffer",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous150: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsFtpProtocolHandler> inst = new nsFtpProtocolHandler();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous035: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/BrowserElementParent.jsm"),
                                    "BrowserElementParent",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous080: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozHunspell>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous382: {
      RefPtr<nsSecurityConsoleMessage> inst = new nsSecurityConsoleMessage();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous387: {
      RefPtr<nsSupportsFloat> inst = new nsSupportsFloat();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous472: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::OnlineSpeechRecognitionService> inst = new mozilla::OnlineSpeechRecognitionService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous463: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<ScriptableContentIterator> inst = new ScriptableContentIterator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous432: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ LocalStorageManagerConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous389: {
      RefPtr<nsSupportsPRBool> inst = new nsSupportsPRBool();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous342: {
      RefPtr<nsFind> inst = new nsFind();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous453: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsJSProtocolHandler> inst = new nsJSProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous145: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsAboutCache::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous407: {
      return /* legacy */ nsMultiplexInputStreamConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous005: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/BrowserGlue.jsm"),
                                    "ContentPermissionPrompt",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous135: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsIncrementalStreamLoader::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous212: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsPK11TokenDB>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous300: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/Prompter.jsm"),
                                    "AuthPromptAdapterFactory",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous218: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsCryptoHMAC>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous105: {
      RefPtr<nsJARProtocolHandler> inst = nsJARProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsJARProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsJARProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsJARProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous313: {
      MOZ_TRY(CallInitFunc(5));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsITelemetry>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous245: {
      RefPtr<mozilla::ContentBlockingTelemetryService> inst = mozilla::ContentBlockingTelemetryService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::ContentBlockingTelemetryService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::ContentBlockingTelemetryService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::ContentBlockingTelemetryService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous241: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TrackingDBService.jsm"),
                                    "TrackingDBService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous272: {
      RefPtr<nsMediaSniffer> inst = new nsMediaSniffer();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous290: {
      RefPtr<nsNavHistory> inst = nsNavHistory::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsNavHistory::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsNavHistory::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsNavHistory, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous114: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::LoadContextInfoFactory> inst = new mozilla::net::LoadContextInfoFactory();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::DBusHandlerApp: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsDBusHandlerApp> inst = new nsDBusHandlerApp();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous164: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSimpleURI::Mutator> inst = new mozilla::net::nsSimpleURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous179: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewMultiMixedConvFactory(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous378: {
      RefPtr<nsMemoryInfoDumper> inst = new nsMemoryInfoDumper();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous375: {
      RefPtr<nsHashPropertyBagCC> inst = new nsHashPropertyBagCC();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous062: {
      RefPtr<mozilla::dom::PaymentAbortActionResponse> inst = new mozilla::dom::PaymentAbortActionResponse();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::PermissionManager: {
      RefPtr<nsIPermissionManager> inst = mozilla::PermissionManager::GetXPCOMSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::PermissionManager::GetXPCOMSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::PermissionManager::GetXPCOMSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIPermissionManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous425: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsContentSecurityManager> inst = new nsContentSecurityManager();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous081: {
      RefPtr<mozPersonalDictionary> inst = new mozPersonalDictionary();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous452: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::OSFileConstantsService> inst = mozilla::OSFileConstantsService::GetOrCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::OSFileConstantsService::GetOrCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::OSFileConstantsService::GetOrCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::OSFileConstantsService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous190: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSimpleURI::Mutator> inst = new mozilla::net::nsSimpleURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous246: {
      RefPtr<nsAutoCompleteController> inst = new nsAutoCompleteController();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous372: {
      RefPtr<nsBinaryOutputStream> inst = new nsBinaryOutputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous371: {
      RefPtr<nsBinaryInputStream> inst = new nsBinaryInputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous134: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ net_NewIncrementalDownload(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous222: {
      RefPtr<nsCertOverrideService> inst = new nsCertOverrideService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous038: {
      RefPtr<mozilla::dom::BlobURLProtocolHandler> inst = new mozilla::dom::BlobURLProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous001: {
      RefPtr<nsGNOMEShellService> inst = new nsGNOMEShellService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous220: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsKeyObjectFactory>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous215: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsNSSCertificateDB>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous082: {
      RefPtr<nsThebesFontEnumerator> inst = new nsThebesFontEnumerator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous330: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/amContentHandler.jsm"),
                                    "amContentHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous012: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/AboutNewTabService.jsm"),
                                    "AboutNewTabStubService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous066: {
      RefPtr<mozilla::dom::PaymentRequestService> inst = mozilla::dom::PaymentRequestService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::PaymentRequestService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::PaymentRequestService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::PaymentRequestService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous143: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsProtocolProxyService> inst = new mozilla::net::nsProtocolProxyService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous405: {
      return /* legacy */ nsArrayBase::XPCOMConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous306: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/InputListAutoComplete.jsm"),
                                    "InputListAutoComplete",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous170: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::TLSServerSocket> inst = new mozilla::net::TLSServerSocket();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous403: {
      return /* legacy */ nsCategoryManager::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous203: {
      MOZ_TRY(CallInitFunc(4));
      RefPtr<nsParser> inst = new nsParser();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous089: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<nsJPEGEncoder> inst = new nsJPEGEncoder();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous015: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://webcompat/AboutCompat.jsm"),
                                    "AboutCompat",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous443: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<inDeepTreeWalker> inst = new inDeepTreeWalker();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous307: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/FormHistoryStartup.jsm"),
                                    "FormHistoryStartup",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous127: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsINativeDNSResolverOverride> inst = mozilla::net::NativeDNSResolverOverride::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::NativeDNSResolverOverride::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::NativeDNSResolverOverride::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsINativeDNSResolverOverride, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous204: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("chrome://remote/content/components/RemoteAgent.jsm"),
                                    "RemoteAgentFactory",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous162: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsServerSocket> inst = new mozilla::net::nsServerSocket();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous461: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::ContentPrincipal> inst = new mozilla::ContentPrincipal();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous404: {
      return /* legacy */ nsComponentManagerImpl::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous055: {
      RefPtr<mozilla::dom::nsFakeSynthServices> inst = mozilla::dom::nsFakeSynthServices::GetInstanceForService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::nsFakeSynthServices::GetInstanceForService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::nsFakeSynthServices::GetInstanceForService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::nsFakeSynthServices, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous194: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsWifiMonitor> inst = new nsWifiMonitor();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous465: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ Construct_nsIScriptSecurityManager(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous110: {
      RefPtr<nsZipWriter> inst = new nsZipWriter();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous075: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PushComponents.jsm"),
                                    "Service",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous094: {
      RefPtr<mozilla::intl::OSPreferences> inst = mozilla::intl::OSPreferences::GetInstanceAddRefed();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::intl::OSPreferences::GetInstanceAddRefed())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::intl::OSPreferences::GetInstanceAddRefed())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::intl::OSPreferences, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous283: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/storage-json.js"),
                                    "LoginManagerStorage_json",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous349: {
      MOZ_TRY(CallInitFunc(6));
      return /* legacy */ nsAppShellConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous040: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozilla::PeerConnectionImpl>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous147: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsAboutProtocolHandler> inst = new mozilla::net::nsAboutProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous219: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsKeyObject>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous459: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsIPresentationService> inst = NS_CreatePresentationService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(NS_CreatePresentationService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(NS_CreatePresentationService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIPresentationService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous044: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "PeerConnectionObserver",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous034: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/SlowScriptDebug.jsm"),
                                    "SlowScriptDebug",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous228: {
      return /* legacy */ cert_storage_constructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous451: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::ReferrerInfo> inst = new mozilla::dom::ReferrerInfo();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous287: {
      RefPtr<nsAnnoProtocolHandler> inst = new nsAnnoProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous333: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UpdateService.jsm"),
                                    "UpdateService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous411: {
      return /* legacy */ nsEnvironment::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous435: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ mozilla::dom::SDBConnection::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous117: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsAtomicFileOutputStream> inst = new nsAtomicFileOutputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous207: {
      RefPtr<nsNSSComponent> inst = new nsNSSComponent();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous177: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewUnknownDecoderFactory(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous393: {
      RefPtr<nsSupportsPRTime> inst = new nsSupportsPRTime();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous213: {
      return /* legacy */ mozilla::psm::NSSConstructor<mozilla::psm::PKCS11ModuleDB>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous107: {
      RefPtr<nsJARURI::Mutator> inst = new nsJARURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous365: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsPrintSession> inst = new nsPrintSession();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::LoginReputation: {
      RefPtr<mozilla::LoginReputationService> inst = mozilla::LoginReputationService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::LoginReputationService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::LoginReputationService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::LoginReputationService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous469: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::net::WebSocketEventService> inst = mozilla::net::WebSocketEventService::GetOrCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::WebSocketEventService::GetOrCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::WebSocketEventService::GetOrCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::net::WebSocketEventService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::DocLoader: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsDocLoader> inst = new nsDocLoader();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::ChromeRegistry: {
      RefPtr<nsChromeRegistry> inst = nsChromeRegistry::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsChromeRegistry::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsChromeRegistry::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsChromeRegistry, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous111: {
      MOZ_TRY(CallInitFunc(2));
      RefPtr<mozilla::Preferences> inst = mozilla::Preferences::GetInstanceForService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::Preferences::GetInstanceForService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::Preferences::GetInstanceForService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::Preferences, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous068: {
      RefPtr<nsPluginHost> inst = nsPluginHost::GetInst();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsPluginHost::GetInst())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsPluginHost::GetInst())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsPluginHost, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous395: {
      RefPtr<nsSupportsPRUint32> inst = new nsSupportsPRUint32();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous356: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsImageToPixbuf> inst = new nsImageToPixbuf();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous026: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsOSPermissionRequest> inst = new nsOSPermissionRequest();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous467: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsIPrincipal> inst = nsScriptSecurityManager::SystemPrincipalSingletonConstructor();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsScriptSecurityManager::SystemPrincipalSingletonConstructor())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsScriptSecurityManager::SystemPrincipalSingletonConstructor())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIPrincipal, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous154: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::PageThumbProtocolHandler> inst = mozilla::net::PageThumbProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::PageThumbProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::PageThumbProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::net::PageThumbProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous396: {
      RefPtr<nsSupportsPRUint64> inst = new nsSupportsPRUint64();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous244: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PartitioningExceptionListService.jsm"),
                                    "PartitioningExceptionListService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous120: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewBinaryDetectorFactory(nullptr, aIID, aResult);
    }
    case ModuleID::ApplicationReputation: {
      RefPtr<ApplicationReputationService> inst = ApplicationReputationService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(ApplicationReputationService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(ApplicationReputationService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<ApplicationReputationService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous090: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<nsPNGEncoder> inst = new nsPNGEncoder();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous237: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://services-sync/Weave.jsm"),
                                    "AboutWeaveLog",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous468: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::TextInputProcessor> inst = new mozilla::TextInputProcessor();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous171: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsUDPSocket> inst = new mozilla::net::nsUDPSocket();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous043: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCPeerConnection",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous357: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsTransferable> inst = new nsTransferable();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous027: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<mozilla::dom::ContentHandlerService> inst = new mozilla::dom::ContentHandlerService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous057: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/NotificationStorage.jsm"),
                                    "NotificationStorage",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::AlertNotification: {
      RefPtr<mozilla::AlertNotification> inst = new mozilla::AlertNotification();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::StringBundle: {
      RefPtr<nsStringBundleService> inst = new nsStringBundleService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous181: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewTXTToHTMLConvFactory(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous056: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/WebVTTParserWrapper.jsm"),
                                    "WebVTTParserWrapper",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous398: {
      RefPtr<nsSupportsString> inst = new nsSupportsString();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous191: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsICookieService> inst = mozilla::net::CookieService::GetXPCOMSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::CookieService::GetXPCOMSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::CookieService::GetXPCOMSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsICookieService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous258: {
      RefPtr<nsICascadeFilter> inst = mozilla::ConstructCascadeFilter();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::ConstructCascadeFilter())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::ConstructCascadeFilter())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsICascadeFilter, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous428: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsDataDocumentContentPolicy> inst = new nsDataDocumentContentPolicy();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous045: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCDTMFSender",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous176: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsIndexedToHTML::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous236: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://services-sync/Weave.jsm"),
                                    "WeaveService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous275: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/MozProtocolHandler.jsm"),
                                    "MozProtocolHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous415: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIFrameTraversal>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous327: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/addonManager.js"),
                                    "BlocklistService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous447: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateHTMLContentSerializer(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous291: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TaggingService.jsm"),
                                    "TaggingService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous363: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsDeviceContextSpecGTK> inst = new nsDeviceContextSpecGTK();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous077: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/NetworkGeolocationProvider.jsm"),
                                    "NetworkGeolocationProvider",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous240: {
      RefPtr<mozilla::storage::VacuumManager> inst = mozilla::storage::VacuumManager::getSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::storage::VacuumManager::getSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::storage::VacuumManager::getSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::storage::VacuumManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous182: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsNestedAboutURI::Mutator> inst = new mozilla::net::nsNestedAboutURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous243: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PurgeTrackerService.jsm"),
                                    "PurgeTrackerService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous392: {
      RefPtr<nsSupportsPRInt64> inst = new nsSupportsPRInt64();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous188: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::SubstitutingURL::Mutator> inst = new mozilla::net::SubstitutingURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous086: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<mozilla::image::imgTools> inst = new mozilla::image::imgTools();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous231: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozISandboxReporter>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous020: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/DevToolsStartup.jsm"),
                                    "DevToolsStartup",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous010: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/ChromeProfileMigrator.jsm"),
                                    "ChromeBetaMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous202: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UrlClassifierExceptionListService.jsm"),
                                    "UrlClassifierExceptionListService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous119: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::BackgroundFileSaverStreamListener> inst = new mozilla::net::BackgroundFileSaverStreamListener();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous358: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsApplicationChooser> inst = new nsApplicationChooser();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous139: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsMIMEHeaderParamImpl> inst = new nsMIMEHeaderParamImpl();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous065: {
      RefPtr<mozilla::dom::PaymentCompleteActionResponse> inst = new mozilla::dom::PaymentCompleteActionResponse();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::ExtensionPolicy: {
      RefPtr<mozilla::ExtensionPolicyService> inst = mozilla::ExtensionPolicyService::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::ExtensionPolicyService::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::ExtensionPolicyService::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::ExtensionPolicyService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous326: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/HelperAppDlg.jsm"),
                                    "nsUnknownContentTypeDialog",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous433: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ SessionStorageManagerConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous232: {
      RefPtr<mozIAppServicesLogger> inst = mozilla::appservices::NewLogService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::appservices::NewLogService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::appservices::NewLogService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozIAppServicesLogger, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous374: {
      RefPtr<nsConsoleService> inst = new nsConsoleService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous449: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsIMediaManagerService> inst = mozilla::MediaManager::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::MediaManager::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::MediaManager::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIMediaManagerService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous437: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIEventListenerService>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous248: {
      RefPtr<nsWebBrowserContentPolicy> inst = new nsWebBrowserContentPolicy();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous189: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::SubstitutingJARURI> inst = new mozilla::net::SubstitutingJARURI();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous256: {
      RefPtr<mozilla::nsTerminator> inst = new mozilla::nsTerminator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous352: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsColorPicker> inst = new nsColorPicker();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous410: {
      return /* legacy */ nsStringInputStreamConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous053: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "CreateOfferRequest",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous344: {
      RefPtr<nsWindowWatcher> inst = new nsWindowWatcher();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous276: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ShieldContentProcess.jsm"),
                                    "AboutStudies",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous186: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::SubstitutingURL::Mutator> inst = new mozilla::net::SubstitutingURL::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous018: {
      RefPtr<mozilla::jsinspector::nsJSInspector> inst = new mozilla::jsinspector::nsJSInspector();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous140: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsMIMEInputStreamConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous286: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/pdfjs.js"),
                                    "StreamConverterFactory",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous180: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewFTPDirListingConv(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous125: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::Dashboard> inst = new mozilla::net::Dashboard();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous274: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/mozIntl.jsm"),
                                    "MozIntl",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous462: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::PushNotifier> inst = new mozilla::dom::PushNotifier();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous205: {
      RefPtr<nsISupports> inst = GetRemoteAgentHandler();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(GetRemoteAgentHandler())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(GetRemoteAgentHandler())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsISupports, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous269: {
      RefPtr<mozIExtensionStorageArea> inst = mozilla::extensions::storage::NewSyncArea();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::extensions::storage::NewSyncArea())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::extensions::storage::NewSyncArea())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozIExtensionStorageArea, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::AppStartup: {
      RefPtr<nsAppStartup> inst = new nsAppStartup();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::AddonContentPolicy: {
      RefPtr<AddonContentPolicy> inst = new AddonContentPolicy();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Prefetch: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsPrefetchService> inst = new nsPrefetchService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous379: {
      RefPtr<nsMemoryReporterManager> inst = new nsMemoryReporterManager();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous460: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::PresentationTCPSessionTransport> inst = new mozilla::dom::PresentationTCPSessionTransport();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous161: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsSerializationHelper> inst = new nsSerializationHelper();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous279: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginAutoComplete.jsm"),
                                    "LoginAutoComplete",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous277: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginManager.jsm"),
                                    "LoginManager",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous310: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/Sidebar.jsm"),
                                    "nsSidebar",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous223: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsRandomGenerator>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous401: {
      RefPtr<nsVariantCC> inst = new nsVariantCC();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous369: {
      return /* legacy */ nsMemoryImpl::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous050: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCPeerConnectionStatic",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous144: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsAboutBlank::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous386: {
      RefPtr<nsSupportsDouble> inst = new nsSupportsDouble();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::NativeFileWatcher: {
      RefPtr<mozilla::NativeFileWatcherService> inst = new mozilla::NativeFileWatcherService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::ServiceWorkerManager: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::ServiceWorkerManager> inst = mozilla::dom::ServiceWorkerManager::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::ServiceWorkerManager::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::ServiceWorkerManager::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::ServiceWorkerManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous351: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<TaskbarProgress> inst = new TaskbarProgress();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous355: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsISound> inst = nsSound::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsSound::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsSound::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsISound, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous151: {
      MOZ_TRY(CallInitFunc(3));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozilla::net::nsHttpHandler>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous113: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<ArrayBufferInputStream> inst = new ArrayBufferInputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous455: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::NullPrincipal> inst = new mozilla::NullPrincipal();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous009: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/FirefoxProfileMigrator.jsm"),
                                    "FirefoxProfileMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous281: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginInfo.jsm"),
                                    "nsLoginInfo",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous426: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsCSPContext> inst = new nsCSPContext();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous312: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TelemetryControllerContent.jsm"),
                                    "getTelemetryController",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous280: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/LoginManagerAuthPrompter.jsm"),
                                    "LoginManagerAuthPrompter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous097: {
      RefPtr<nsConverterOutputStream> inst = new nsConverterOutputStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous146: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsAboutCacheEntry> inst = new nsAboutCacheEntry();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous259: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ClearDataService.jsm"),
                                    "ClearDataService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous214: {
      return /* legacy */ mozilla::psm::NSSConstructor<nsNSSCertificate>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous072: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PresentationControlService.jsm"),
                                    "PresentationControlService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous373: {
      RefPtr<nsChromeProtocolHandler> inst = new nsChromeProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous253: {
      RefPtr<nsUpdateProcessor> inst = new nsUpdateProcessor();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous288: {
      RefPtr<nsFaviconService> inst = nsFaviconService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsFaviconService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsFaviconService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsFaviconService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous397: {
      RefPtr<nsSupportsPRUint8> inst = new nsSupportsPRUint8();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous121: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsBufferedInputStream::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous109: {
      RefPtr<nsDeflateConverter> inst = new nsDeflateConverter();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous155: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsSafeAboutProtocolHandler> inst = new mozilla::net::nsSafeAboutProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous464: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsScriptError> inst = new nsScriptError();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous095: {
      RefPtr<nsCollationFactory> inst = new nsCollationFactory();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous064: {
      RefPtr<mozilla::dom::PaymentCanMakeActionResponse> inst = new mozilla::dom::PaymentCanMakeActionResponse();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous092: {
      RefPtr<nsMozIconURI::Mutator> inst = new nsMozIconURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous104: {
      RefPtr<nsJAR> inst = new nsJAR();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous444: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateContentPolicy(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous004: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/BrowserGlue.jsm"),
                                    "BrowserGlue",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous013: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/PromptCollection.jsm"),
                                    "PromptCollection",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous311: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TelemetryStartup.jsm"),
                                    "TelemetryStartup",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::UrlClassifierStreamUpdater: {
      RefPtr<nsUrlClassifierStreamUpdater> inst = new nsUrlClassifierStreamUpdater();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::UUIDGenerator: {
      RefPtr<nsUUIDGenerator> inst = new nsUUIDGenerator();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous340: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsUnixSystemProxySettings>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous122: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsBufferedOutputStream::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous257: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/CaptiveDetect.jsm"),
                                    "CaptivePortalDetector",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous384: {
      RefPtr<nsSupportsCString> inst = new nsSupportsCString();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous061: {
      RefPtr<mozilla::dom::GeneralResponseData> inst = new mozilla::dom::GeneralResponseData();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous123: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsCacheServiceConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous063: {
      RefPtr<mozilla::dom::payments::PaymentAddress> inst = new mozilla::dom::payments::PaymentAddress();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous377: {
      return /* legacy */ nsLocalFileConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous261: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ContentPrefService2.jsm"),
                                    "ContentPrefService2",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous025: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsExternalProtocolHandler> inst = new nsExternalProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous054: {
      RefPtr<mozilla::dom::SpeechDispatcherService> inst = mozilla::dom::SpeechDispatcherService::GetInstanceForService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::SpeechDispatcherService::GetInstanceForService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::SpeechDispatcherService::GetInstanceForService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::SpeechDispatcherService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous101: {
      RefPtr<mozilla::jsdebugger::JSDebugger> inst = new mozilla::jsdebugger::JSDebugger();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous450: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsMixedContentBlocker> inst = new nsMixedContentBlocker();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous041: {
      RefPtr<nsStunTCPSocketFilterHandler> inst = new nsStunTCPSocketFilterHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous250: {
      RefPtr<mozilla::FinalizationWitnessService> inst = new mozilla::FinalizationWitnessService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous370: {
      return /* legacy */ nsMessageLoopConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous102: {
      RefPtr<mozJSSubScriptLoader> inst = new mozJSSubScriptLoader();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous331: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/amWebAPI.jsm"),
                                    "WebAPI",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous028: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<PlatformLocalHandlerApp_t> inst = new PlatformLocalHandlerApp_t();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous454: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsNoDataProtocolContentPolicy> inst = new nsNoDataProtocolContentPolicy();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous116: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsAsyncStreamCopier> inst = new nsAsyncStreamCopier();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous137: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsInputStreamPump> inst = new nsInputStreamPump();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous152: {
      MOZ_TRY(CallInitFunc(3));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<mozilla::net::nsHttpsHandler>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous242: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/URLDecorationAnnotationsService.jsm"),
                                    "URLDecorationAnnotationsService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous209: {
      RefPtr<nsNSSVersion> inst = new nsNSSVersion();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous008: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/ChromeProfileMigrator.jsm"),
                                    "ChromiumProfileMigrator",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous091: {
      RefPtr<nsIconProtocolHandler> inst = new nsIconProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous457: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsParserUtils> inst = new nsParserUtils();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous260: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/DefaultCLH.jsm"),
                                    "nsDefaultCLH",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous211: {
      return /* legacy */ mozilla::psm::NSSConstructor<SecretDecoderRing>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous292: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TaggingService.jsm"),
                                    "TagAutoCompleteSearch",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous318: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UrlClassifierLib.jsm"),
                                    "UrlClassifierLib",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous049: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/media/PeerConnection.jsm"),
                                    "RTCStatsReport",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous071: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/PresentationDataChannelSessionTransport.jsm"),
                                    "PresentationTransportBuilder",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous024: {
      MOZ_TRY(CallInitFunc(0));
      return /* legacy */ nsAboutRedirector::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous336: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UpdateServiceStub.jsm"),
                                    "UpdateServiceStub",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous153: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::ExtensionProtocolHandler> inst = mozilla::net::ExtensionProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::ExtensionProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::ExtensionProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::net::ExtensionProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous417: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsJSURI::Mutator> inst = new nsJSURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous226: {
      return /* legacy */ mozilla::psm::NSSConstructor<OSKeyStore>(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous098: {
      RefPtr<nsScriptableUnicodeConverter> inst = new nsScriptableUnicodeConverter();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous471: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::nsSynthVoiceRegistry> inst = mozilla::dom::nsSynthVoiceRegistry::GetInstanceForService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::nsSynthVoiceRegistry::GetInstanceForService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::nsSynthVoiceRegistry::GetInstanceForService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::nsSynthVoiceRegistry, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous251: {
      RefPtr<mozilla::NativeOSFileInternalsService> inst = new mozilla::NativeOSFileInternalsService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous328: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/addonManager.js"),
                                    "amManager",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous324: {
      RefPtr<mozilla::Viaduct> inst = mozilla::Viaduct::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::Viaduct::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::Viaduct::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::Viaduct, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous167: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsStreamListenerTee> inst = new mozilla::net::nsStreamListenerTee();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous364: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsPrintDialogServiceGTK> inst = new nsPrintDialogServiceGTK();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous031: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ContentAreaDropListener.jsm"),
                                    "ContentAreaDropListener",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous470: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::EventSourceEventService> inst = mozilla::dom::EventSourceEventService::GetOrCreate();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::EventSourceEventService::GetOrCreate())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::EventSourceEventService::GetOrCreate())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::EventSourceEventService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::URIFixup: {
      MOZ_TRY(CallInitFunc(0));
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/URIFixup.jsm"),
                                    "URIFixup",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::TypeAheadFind: {
      RefPtr<nsTypeAheadFind> inst = new nsTypeAheadFind();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous133: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsIDNService> inst = new nsIDNService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous418: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIContentViewer>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Alerts: {
      RefPtr<nsAlertsService> inst = new nsAlertsService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous339: {
      RefPtr<nsSystemAlertsService> inst = new nsSystemAlertsService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous099: {
      RefPtr<nsTextToSubURI> inst = new nsTextToSubURI();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous314: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/TerminatorTelemetry.jsm"),
                                    "nsTerminatorTelemetry",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous185: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsNestedAboutURI::Mutator> inst = new mozilla::net::nsNestedAboutURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous282: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/crypto-SDR.js"),
                                    "LoginManagerCrypto_SDR",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous304: {
      RefPtr<nsFormFillController> inst = nsFormFillController::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsFormFillController::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsFormFillController::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsFormFillController, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous354: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsHTMLFormatConverter> inst = new nsHTMLFormatConverter();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous000: {
      return /* legacy */ mozilla::browser::AboutRedirector::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous301: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/Prompter.jsm"),
                                    "Prompter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous073: {
      RefPtr<mozilla::dom::presentation::MulticastDNSDeviceProvider> inst = new mozilla::dom::presentation::MulticastDNSDeviceProvider();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous414: {
      RefPtr<nsWindowMediator> inst = new nsWindowMediator();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous078: {
      RefPtr<PermissionDelegateHandler> inst = new PermissionDelegateHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous002: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/BrowserContentHandler.jsm"),
                                    "nsBrowserContentHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous422: {
      MOZ_TRY(CallInitFunc(7));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIDocumentLoaderFactory>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous088: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<nsICOEncoder> inst = new nsICOEncoder();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous136: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsInputStreamChannel> inst = new mozilla::net::nsInputStreamChannel();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous297: {
      RefPtr<mozISyncedBookmarksMerger> inst = mozilla::places::NewSyncedBookmarksMerger();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::places::NewSyncedBookmarksMerger())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::places::NewSyncedBookmarksMerger())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozISyncedBookmarksMerger, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous023: {
      MOZ_TRY(CallInitFunc(0));
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/URIFixup.jsm"),
                                    "URIFixupInfo",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous341: {
      return /* legacy */ mozilla::AppInfoConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous252: {
      RefPtr<nsBrowserStatusFilter> inst = new nsBrowserStatusFilter();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous321: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/URLFormatter.jsm"),
                                    "nsURLFormatterService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous406: {
      return /* legacy */ nsObserverService::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous264: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/nsCrashMonitor.jsm"),
                                    "CrashMonitor",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous169: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsIInputChannelThrottleQueue> inst = mozilla::net::ThrottleQueue::Create();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::ThrottleQueue::Create())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::ThrottleQueue::Create())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIInputChannelThrottleQueue, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous132: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsHttpAuthManager> inst = new mozilla::net::nsHttpAuthManager();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous285: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/AutoCompleteSimpleSearch.jsm"),
                                    "AutoCompleteSimpleSearch",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous473: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::FakeSpeechRecognitionService> inst = new mozilla::FakeSpeechRecognitionService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous345: {
      RefPtr<nsXREDirProvider> inst = nsXREDirProvider::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsXREDirProvider::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsXREDirProvider::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsXREDirProvider, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous022: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource:///modules/AboutDevToolsToolboxRegistration.jsm"),
                                    "AboutDevtoolsToolbox",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous421: {
      MOZ_TRY(CallInitFunc(7));
      return /* legacy */ CreateChildMessageManager(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous366: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsPrintSettingsServiceGTK> inst = new nsPrintSettingsServiceGTK();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::AsyncShutdown: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/nsAsyncShutdown.jsm"),
                                    "nsAsyncShutdownService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous302: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/Prompter.jsm"),
                                    "EmbedPrompter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::UrlClassifierPrefixSet: {
      RefPtr<nsUrlClassifierPrefixSet> inst = new nsUrlClassifierPrefixSet();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous334: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/UpdateService.jsm"),
                                    "UpdateManager",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous299: {
      RefPtr<nsIProcessToolsService> inst = GetProcessToolsService();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(GetProcessToolsService())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(GetProcessToolsService())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIProcessToolsService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous394: {
      RefPtr<nsSupportsPRUint16> inst = new nsSupportsPRUint16();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous305: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/FormAutoComplete.jsm"),
                                    "FormAutoComplete",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous129: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsEffectiveTLDService> inst = new nsEffectiveTLDService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous033: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ProcessSelector.jsm"),
                                    "MinTabSelector",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous083: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<imgLoader> inst = new imgLoader();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::DownloadPlatform: {
      RefPtr<DownloadPlatform> inst = new DownloadPlatform();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous168: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ mozilla::net::nsStreamLoader::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous148: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ nsDataHandler::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous376: {
      RefPtr<nsIOUtil> inst = new nsIOUtil();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous037: {
      RefPtr<mozilla::dom::FontTableURIProtocolHandler> inst = new mozilla::dom::FontTableURIProtocolHandler();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous412: {
      return /* legacy */ nsTimer::XPCOMConstructor(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous329: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/amInstallTrigger.jsm"),
                                    "InstallTrigger",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous112: {
      MOZ_TRY(CallInitFunc(2));
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsPrefLocalizedString>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous210: {
      RefPtr<mozilla::LocalCertService> inst = new mozilla::LocalCertService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous296: {
      RefPtr<mozilla::places::PageIconProtocolHandler> inst = mozilla::places::PageIconProtocolHandler::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::places::PageIconProtocolHandler::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::places::PageIconProtocolHandler::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::places::PageIconProtocolHandler, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous416: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<nsJSURI::Mutator> inst = new nsJSURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous030: {
      MOZ_TRY(CallInitFunc(0));
      RefPtr<nsExternalHelperAppService> inst = nsExternalHelperAppService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsExternalHelperAppService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsExternalHelperAppService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsExternalHelperAppService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous348: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/WebHandlerApp.jsm"),
                                    "nsWebHandlerApp",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous409: {
      return /* legacy */ nsScriptableInputStream::Create(nullptr, aIID, aResult);
    }
    case ModuleID::StreamTransport: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::nsStreamTransportService> inst = new mozilla::net::nsStreamTransportService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous238: {
      nsCOMPtr<nsISupports> inst = mozCreateComponent<nsIStartupCacheInfo>();
      NS_ENSURE_TRUE(inst, NS_ERROR_FAILURE);
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous017: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://devtools/client/jsonview/Converter.jsm"),
                                    "Converter",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous385: {
      RefPtr<nsSupportsChar> inst = new nsSupportsChar();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous200: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/WellKnownOpportunisticUtils.jsm"),
                                    "WellKnownOpportunisticUtils",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::CacheStorage: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<mozilla::net::CacheStorageService> inst = new mozilla::net::CacheStorageService();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous270: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ExtensionStorageComponents.jsm"),
                                    "StorageSyncService",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous338: {
      RefPtr<nsGSettingsService> inst = new nsGSettingsService();
      MOZ_TRY(inst->Init());
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous353: {
      MOZ_TRY(CallInitFunc(6));
      RefPtr<nsFilePicker> inst = new nsFilePicker();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous383: {
      RefPtr<nsStorageStream> inst = new nsStorageStream();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous436: {
      MOZ_TRY(CallInitFunc(7));
      RefPtr<mozilla::dom::WorkerDebuggerManager> inst = mozilla::dom::WorkerDebuggerManager::GetInstance();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::dom::WorkerDebuggerManager::GetInstance())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::dom::WorkerDebuggerManager::GetInstance())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::dom::WorkerDebuggerManager, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous178: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ CreateNewHTTPCompressConvFactory(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous201: {
      RefPtr<nsIChannelClassifierService> inst = mozilla::net::ChannelClassifierService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::ChannelClassifierService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::ChannelClassifierService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIChannelClassifierService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous085: {
      MOZ_TRY(CallInitFunc(1));
      RefPtr<imgRequestProxy> inst = new imgRequestProxy();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous124: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsICaptivePortalService> inst = mozilla::net::CaptivePortalService::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::net::CaptivePortalService::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::net::CaptivePortalService::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsICaptivePortalService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::History: {
      RefPtr<mozilla::places::History> inst = mozilla::places::History::GetSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(mozilla::places::History::GetSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(mozilla::places::History::GetSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<mozilla::places::History, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous142: {
      MOZ_TRY(CallInitFunc(3));
      return /* legacy */ mozilla::net::Predictor::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous323: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/SimpleServices.jsm"),
                                    "HttpIndexViewer",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous266: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/DotProtocolHandler.jsm"),
                                    "DotProtocolHandler",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous106: {
      RefPtr<nsJARURI::Mutator> inst = new nsJARURI::Mutator();
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous268: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/EnterprisePolicies.jsm"),
                                    "EnterprisePolicies",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous126: {
      MOZ_TRY(CallInitFunc(3));
      RefPtr<nsIDNSService> inst = nsDNSService::GetXPCOMSingleton();
      NS_ENSURE_TRUE(inst, NS_ERROR_OUT_OF_MEMORY);

      using T =
          RemoveAlreadyAddRefed<decltype(nsDNSService::GetXPCOMSingleton())>::Type;
      static_assert(
          std::is_same_v<already_AddRefed<T>, decltype(nsDNSService::GetXPCOMSingleton())>,
          "Singleton constructor must return already_AddRefed");
      static_assert(
          std::is_base_of<nsIDNSService, T>::value,
          "Singleton constructor must return correct already_AddRefed");

      return inst->QueryInterface(aIID, aResult);
    }
    case ModuleID::Anonymous368: {
      return /* legacy */ nsDebugImpl::Create(nullptr, aIID, aResult);
    }
    case ModuleID::Anonymous332: {
      nsCOMPtr<nsISupports> inst;
      MOZ_TRY(ConstructJSMComponent(nsLiteralCString("resource://gre/modules/ContentDispatchChooser.jsm"),
                                    "nsContentDispatchChooser",
                                    getter_AddRefs(inst)));
      return inst->QueryInterface(aIID, aResult);
    }
  }

  MOZ_ASSERT_UNREACHABLE("Constructor didn't return");
  return NS_ERROR_FAILURE;
}


namespace {

class StaticModuleFactory final : public nsIFactory {
  NS_DECL_ISUPPORTS
  NS_DECL_NSIFACTORY

  explicit StaticModuleFactory(ModuleID aID) : mID(aID) {}

private:
  ~StaticModuleFactory() = default;

  const ModuleID mID;
};

NS_IMPL_ISUPPORTS(StaticModuleFactory, nsIFactory)

NS_IMETHODIMP StaticModuleFactory::CreateInstance(nsISupports* aOuter,
                                                  const nsIID& aIID,
                                                  void** aResult) {
  return CreateInstanceImpl(mID, aOuter, aIID, aResult);
}

NS_IMETHODIMP StaticModuleFactory::LockFactory(bool aLock) {
  MOZ_CRASH("LockFactory is no longer a thing");
  return NS_ERROR_NOT_IMPLEMENTED;
}

}  // anonymous namespace


already_AddRefed<nsIFactory> StaticModule::GetFactory() const {
  return do_AddRef(new StaticModuleFactory(ID()));
}

bool StaticModule::Active() const {
  return FastProcessSelectorMatches(mProcessSelector);
}

bool StaticModule::Overridable() const {
  return mContractID.mOffset != kNoContractID;
}

nsCString StaticModule::ContractID() const {
  MOZ_ASSERT(Overridable());
  return GetString(mContractID);
}

nsresult StaticModule::CreateInstance(nsISupports* aOuter, const nsIID& aIID,
                                      void** aResult) const {
  return CreateInstanceImpl(ID(), aOuter, aIID, aResult);
}

GetServiceHelper StaticModule::GetService() const {
  return { ID(), nullptr };
}

GetServiceHelper StaticModule::GetService(nsresult* aRv) const {
  return { ID(), aRv };
}


nsISupports* StaticModule::ServiceInstance() const {
  return gServiceInstances[Idx()];
}

void StaticModule::SetServiceInstance(
    already_AddRefed<nsISupports> aInst) const {
  gServiceInstances[Idx()] = aInst;
}


nsCString StaticCategoryEntry::Entry() const {
  return GetString(mEntry);
}

nsCString StaticCategoryEntry::Value() const {
  return GetString(mValue);
}

bool StaticCategoryEntry::Active() const {
  if (!FastProcessSelectorMatches(mProcessSelector)) {
    return false;
  }
#ifdef MOZ_BACKGROUNDTASKS
  if (MOZ_UNLIKELY(BackgroundTasks::IsBackgroundTaskMode())) {
    return mBackgroundTasksSelector != Module::BackgroundTasksSelector::NO_TASKS;
  }
#endif /* MOZ_BACKGROUNDTASKS */
  return true;
}

nsCString StaticCategory::Name() const {
  return GetString(mName);
}

nsCString JSServiceEntry::Name() const {
  return GetString(mName);
}

JSServiceEntry::InterfaceList JSServiceEntry::Interfaces() const {
  InterfaceList iids;
  iids.SetCapacity(mInterfaceCount);

  for (size_t i = 0; i < mInterfaceCount; i++) {
    nsXPTInterface ifaceID = gInterfaces[mInterfaceOffset.mOffset + i];
    iids.AppendElement(&nsXPTInterfaceInfo::Get(ifaceID)->IID());
  }
  return iids;
}


/* static */
const JSServiceEntry* JSServiceEntry::Lookup(const nsACString& aName) {
  return LookupJSService(aName);
}

/* static */ const StaticModule* StaticComponents::LookupByCID(
    const nsID& aCID) {
  return ModuleByCID(aCID);
}

/* static */ const StaticModule* StaticComponents::LookupByContractID(
    const nsACString& aContractID) {
  if (const ContractEntry* entry = LookupContractID(aContractID)) {
    if (!entry->Invalid()) {
      return &entry->Module();
    }
  }
  return nullptr;
}

/* static */ bool StaticComponents::InvalidateContractID(
    const nsACString& aContractID, bool aInvalid) {
  if (const ContractEntry* entry = LookupContractID(aContractID)) {
    entry->SetInvalid(aInvalid);
    return true;
  }
  return false;
}

/* static */ already_AddRefed<nsIUTF8StringEnumerator>
StaticComponents::GetComponentJSMs() {
  auto jsms = MakeUnique<nsTArray<nsCString>>(MOZ_ARRAY_LENGTH(gComponentJSMs));

  for (const auto& entry : gComponentJSMs) {
    jsms->AppendElement(GetString(entry));
  }

  nsCOMPtr<nsIUTF8StringEnumerator> result;
  MOZ_ALWAYS_SUCCEEDS(NS_NewAdoptingUTF8StringEnumerator(getter_AddRefs(result),
                                                         jsms.release()));
  return result.forget();
}

/* static */ Span<const JSServiceEntry> StaticComponents::GetJSServices() {
  return { gJSServices, ArrayLength(gJSServices) };
}

/* static */ void StaticComponents::Shutdown() {
  CallUnloadFuncs();
}

/* static */ const nsID& Components::GetCID(ModuleID aID) {
  return gStaticModules[size_t(aID)].CID();
}

nsresult GetServiceHelper::operator()(const nsIID& aIID, void** aResult) const {
  nsresult rv =
      nsComponentManagerImpl::gComponentManager->GetService(mId, aIID, aResult);
  return SetResult(rv);
}

nsresult CreateInstanceHelper::operator()(const nsIID& aIID,
                                          void** aResult) const {
  const auto& entry = gStaticModules[size_t(mId)];
  if (!entry.Active()) {
    return SetResult(NS_ERROR_FACTORY_NOT_REGISTERED);
  }

  nsresult rv = entry.CreateInstance(nullptr, aIID, aResult);
  return SetResult(rv);
}

}  // namespace xpcom
}  // namespace mozilla
