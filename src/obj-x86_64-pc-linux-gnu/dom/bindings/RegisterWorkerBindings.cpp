#include "AbortControllerBinding.h"
#include "AbortSignalBinding.h"
#include "BlobBinding.h"
#include "BroadcastChannelBinding.h"
#include "CacheBinding.h"
#include "CacheStorageBinding.h"
#include "ChromeUtilsBinding.h"
#include "ClientBinding.h"
#include "ClientsBinding.h"
#include "CloseEventBinding.h"
#include "ConsoleBinding.h"
#include "CryptoBinding.h"
#include "CustomEventBinding.h"
#include "DOMExceptionBinding.h"
#include "DOMMatrixBinding.h"
#include "DOMPointBinding.h"
#include "DOMQuadBinding.h"
#include "DOMRectBinding.h"
#include "DOMRequestBinding.h"
#include "DOMStringListBinding.h"
#include "DebuggerNotificationObserverBinding.h"
#include "DedicatedWorkerGlobalScopeBinding.h"
#include "DirectoryBinding.h"
#include "DominatorTreeBinding.h"
#include "ErrorEventBinding.h"
#include "EventBinding.h"
#include "EventSourceBinding.h"
#include "EventTargetBinding.h"
#include "ExtendableEventBinding.h"
#include "ExtendableMessageEventBinding.h"
#include "FetchEventBinding.h"
#include "FetchObserverBinding.h"
#include "FileBinding.h"
#include "FileListBinding.h"
#include "FileReaderBinding.h"
#include "FileReaderSyncBinding.h"
#include "FormDataBinding.h"
#include "HeadersBinding.h"
#include "HeapSnapshotBinding.h"
#include "IDBCursorBinding.h"
#include "IDBDatabaseBinding.h"
#include "IDBFactoryBinding.h"
#include "IDBIndexBinding.h"
#include "IDBKeyRangeBinding.h"
#include "IDBObjectStoreBinding.h"
#include "IDBOpenDBRequestBinding.h"
#include "IDBRequestBinding.h"
#include "IDBTransactionBinding.h"
#include "IDBVersionChangeEventBinding.h"
#include "IOUtilsBinding.h"
#include "ImageBitmapBinding.h"
#include "ImageBitmapRenderingContextBinding.h"
#include "ImageDataBinding.h"
#include "MediaCapabilitiesBinding.h"
#include "MessageChannelBinding.h"
#include "MessageEventBinding.h"
#include "MessagePortBinding.h"
#include "NetworkInformationBinding.h"
#include "NotificationBinding.h"
#include "NotificationEventBinding.h"
#include "OffscreenCanvasBinding.h"
#include "PathUtilsBinding.h"
#include "PerformanceBinding.h"
#include "PerformanceEntryBinding.h"
#include "PerformanceMarkBinding.h"
#include "PerformanceMeasureBinding.h"
#include "PerformanceObserverBinding.h"
#include "PerformanceObserverEntryListBinding.h"
#include "PerformanceResourceTimingBinding.h"
#include "PerformanceServerTimingBinding.h"
#include "ProgressEventBinding.h"
#include "PromiseRejectionEventBinding.h"
#include "PushEventBinding.h"
#include "PushManagerBinding.h"
#include "PushMessageDataBinding.h"
#include "PushSubscriptionBinding.h"
#include "PushSubscriptionOptionsBinding.h"
#include "ReportingBinding.h"
#include "RequestBinding.h"
#include "ResponseBinding.h"
#include "ServiceWorkerBinding.h"
#include "ServiceWorkerGlobalScopeBinding.h"
#include "ServiceWorkerRegistrationBinding.h"
#include "SharedWorkerGlobalScopeBinding.h"
#include "StorageManagerBinding.h"
#include "StructuredCloneHolderBinding.h"
#include "StructuredCloneTesterBinding.h"
#include "SubtleCryptoBinding.h"
#include "TextDecoderBinding.h"
#include "TextEncoderBinding.h"
#include "URLBinding.h"
#include "URLSearchParamsBinding.h"
#include "WebGLContextEventBinding.h"
#include "WebGLRenderingContextBinding.h"
#include "WebSocketBinding.h"
#include "WorkerBinding.h"
#include "WorkerGlobalScopeBinding.h"
#include "WorkerLocationBinding.h"
#include "WorkerNavigatorBinding.h"
#include "XMLHttpRequestBinding.h"
#include "XMLHttpRequestEventTargetBinding.h"
#include "XMLHttpRequestUploadBinding.h"

namespace mozilla {
namespace dom {
bool
RegisterWorkerBindings(JSContext* aCx, JS::Handle<JSObject*> aObj)
{
  if (!AbortController_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!AbortSignal_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Blob_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!BroadcastChannel_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Cache_Binding::ConstructorEnabled(aCx, aObj) && !Cache_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (CacheStorage_Binding::ConstructorEnabled(aCx, aObj) && !CacheStorage_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ChromeUtils_Binding::ConstructorEnabled(aCx, aObj) && !ChromeUtils_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ChromeWorker_Binding::ConstructorEnabled(aCx, aObj) && !ChromeWorker_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Client_Binding::ConstructorEnabled(aCx, aObj) && !Client_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Clients_Binding::ConstructorEnabled(aCx, aObj) && !Clients_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!CloseEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ConsoleInstance_Binding::ConstructorEnabled(aCx, aObj) && !ConsoleInstance_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Crypto_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!CustomEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!DOMException_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DOMMatrix_Binding::ConstructorEnabled(aCx, aObj) && !DOMMatrix_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DOMMatrixReadOnly_Binding::ConstructorEnabled(aCx, aObj) && !DOMMatrixReadOnly_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DOMPoint_Binding::ConstructorEnabled(aCx, aObj) && !DOMPoint_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DOMPointReadOnly_Binding::ConstructorEnabled(aCx, aObj) && !DOMPointReadOnly_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DOMQuad_Binding::ConstructorEnabled(aCx, aObj) && !DOMQuad_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!DOMRect_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!DOMRectReadOnly_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!DOMRequest_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!DOMStringList_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DebuggerNotificationObserver_Binding::ConstructorEnabled(aCx, aObj) && !DebuggerNotificationObserver_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DedicatedWorkerGlobalScope_Binding::ConstructorEnabled(aCx, aObj) && !DedicatedWorkerGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Directory_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (DominatorTree_Binding::ConstructorEnabled(aCx, aObj) && !DominatorTree_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!ErrorEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Event_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (EventSource_Binding::ConstructorEnabled(aCx, aObj) && !EventSource_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!EventTarget_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ExtendableEvent_Binding::ConstructorEnabled(aCx, aObj) && !ExtendableEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ExtendableMessageEvent_Binding::ConstructorEnabled(aCx, aObj) && !ExtendableMessageEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (FetchEvent_Binding::ConstructorEnabled(aCx, aObj) && !FetchEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (FetchObserver_Binding::ConstructorEnabled(aCx, aObj) && !FetchObserver_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!File_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!FileList_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!FileReader_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (FileReaderSync_Binding::ConstructorEnabled(aCx, aObj) && !FileReaderSync_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!FormData_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Headers_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (HeapSnapshot_Binding::ConstructorEnabled(aCx, aObj) && !HeapSnapshot_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBCursor_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBCursorWithValue_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBDatabase_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBFactory_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBIndex_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBKeyRange_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (IDBLocaleAwareKeyRange_Binding::ConstructorEnabled(aCx, aObj) && !IDBLocaleAwareKeyRange_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBObjectStore_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBOpenDBRequest_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBRequest_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBTransaction_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!IDBVersionChangeEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (IOUtils_Binding::ConstructorEnabled(aCx, aObj) && !IOUtils_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!ImageBitmap_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!ImageBitmapRenderingContext_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!ImageData_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (MediaCapabilities_Binding::ConstructorEnabled(aCx, aObj) && !MediaCapabilities_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (MediaCapabilitiesInfo_Binding::ConstructorEnabled(aCx, aObj) && !MediaCapabilitiesInfo_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!MessageChannel_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!MessageEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!MessagePort_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (NetworkInformation_Binding::ConstructorEnabled(aCx, aObj) && !NetworkInformation_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Notification_Binding::ConstructorEnabled(aCx, aObj) && !Notification_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (NotificationEvent_Binding::ConstructorEnabled(aCx, aObj) && !NotificationEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (OffscreenCanvas_Binding::ConstructorEnabled(aCx, aObj) && !OffscreenCanvas_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PathUtils_Binding::ConstructorEnabled(aCx, aObj) && !PathUtils_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Performance_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PerformanceEntry_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PerformanceMark_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PerformanceMeasure_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PerformanceObserver_Binding::ConstructorEnabled(aCx, aObj) && !PerformanceObserver_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PerformanceObserverEntryList_Binding::ConstructorEnabled(aCx, aObj) && !PerformanceObserverEntryList_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PerformanceResourceTiming_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PerformanceServerTiming_Binding::ConstructorEnabled(aCx, aObj) && !PerformanceServerTiming_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!ProgressEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!PromiseRejectionEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PushEvent_Binding::ConstructorEnabled(aCx, aObj) && !PushEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PushManager_Binding::ConstructorEnabled(aCx, aObj) && !PushManager_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PushMessageData_Binding::ConstructorEnabled(aCx, aObj) && !PushMessageData_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PushSubscription_Binding::ConstructorEnabled(aCx, aObj) && !PushSubscription_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (PushSubscriptionOptions_Binding::ConstructorEnabled(aCx, aObj) && !PushSubscriptionOptions_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Report_Binding::ConstructorEnabled(aCx, aObj) && !Report_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ReportBody_Binding::ConstructorEnabled(aCx, aObj) && !ReportBody_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ReportingObserver_Binding::ConstructorEnabled(aCx, aObj) && !ReportingObserver_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Request_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!Response_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ServiceWorker_Binding::ConstructorEnabled(aCx, aObj) && !ServiceWorker_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ServiceWorkerGlobalScope_Binding::ConstructorEnabled(aCx, aObj) && !ServiceWorkerGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (ServiceWorkerRegistration_Binding::ConstructorEnabled(aCx, aObj) && !ServiceWorkerRegistration_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (SharedWorkerGlobalScope_Binding::ConstructorEnabled(aCx, aObj) && !SharedWorkerGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (StorageManager_Binding::ConstructorEnabled(aCx, aObj) && !StorageManager_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (StructuredCloneHolder_Binding::ConstructorEnabled(aCx, aObj) && !StructuredCloneHolder_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (StructuredCloneTester_Binding::ConstructorEnabled(aCx, aObj) && !StructuredCloneTester_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (SubtleCrypto_Binding::ConstructorEnabled(aCx, aObj) && !SubtleCrypto_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (TestingDeprecatedInterface_Binding::ConstructorEnabled(aCx, aObj) && !TestingDeprecatedInterface_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!TextDecoder_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!TextEncoder_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!URL_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!URLSearchParams_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLActiveInfo_Binding::ConstructorEnabled(aCx, aObj) && !WebGLActiveInfo_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLBuffer_Binding::ConstructorEnabled(aCx, aObj) && !WebGLBuffer_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLContextEvent_Binding::ConstructorEnabled(aCx, aObj) && !WebGLContextEvent_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLFramebuffer_Binding::ConstructorEnabled(aCx, aObj) && !WebGLFramebuffer_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLProgram_Binding::ConstructorEnabled(aCx, aObj) && !WebGLProgram_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLRenderbuffer_Binding::ConstructorEnabled(aCx, aObj) && !WebGLRenderbuffer_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLRenderingContext_Binding::ConstructorEnabled(aCx, aObj) && !WebGLRenderingContext_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLShader_Binding::ConstructorEnabled(aCx, aObj) && !WebGLShader_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLShaderPrecisionFormat_Binding::ConstructorEnabled(aCx, aObj) && !WebGLShaderPrecisionFormat_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLTexture_Binding::ConstructorEnabled(aCx, aObj) && !WebGLTexture_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WebGLUniformLocation_Binding::ConstructorEnabled(aCx, aObj) && !WebGLUniformLocation_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WebSocket_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (WindowClient_Binding::ConstructorEnabled(aCx, aObj) && !WindowClient_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (Worker_Binding::ConstructorEnabled(aCx, aObj) && !Worker_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WorkerGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WorkerLocation_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WorkerNavigator_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (XMLHttpRequest_Binding::ConstructorEnabled(aCx, aObj) && !XMLHttpRequest_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (XMLHttpRequestEventTarget_Binding::ConstructorEnabled(aCx, aObj) && !XMLHttpRequestEventTarget_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (XMLHttpRequestUpload_Binding::ConstructorEnabled(aCx, aObj) && !XMLHttpRequestUpload_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!console_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  return true;
}

} // namespace dom
} // namespace mozilla

