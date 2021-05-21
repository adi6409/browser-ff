/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/gmp/mozIGeckoMediaPluginService.idl
 */

#ifndef __gen_mozIGeckoMediaPluginService_h__
#define __gen_mozIGeckoMediaPluginService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIThread_h__
#include "nsIThread.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/UniquePtr.h"
#include "nsTArray.h"
#include "nsString.h"
class GMPDecryptorProxy;
class GMPVideoDecoderProxy;
class GMPVideoEncoderProxy;
class GMPVideoHost;
namespace mozilla {
class GMPCrashHelper;
}
template<class T>
class GMPGetterCallback
{
public:
  GMPGetterCallback() { MOZ_COUNT_CTOR(GMPGetterCallback<T>); }
  virtual ~GMPGetterCallback() { MOZ_COUNT_DTOR(GMPGetterCallback<T>); }
  virtual void Done(T*) = 0;
};
template<class T>
class GMPVideoGetterCallback
{
public:
  GMPVideoGetterCallback() { MOZ_COUNT_CTOR(GMPVideoGetterCallback<T>); }
  virtual ~GMPVideoGetterCallback() { MOZ_COUNT_DTOR(GMPVideoGetterCallback<T>); }
  virtual void Done(T*, GMPVideoHost*) = 0;
};
typedef GMPGetterCallback<GMPDecryptorProxy> GetGMPDecryptorCallback;
typedef GMPVideoGetterCallback<GMPVideoDecoderProxy> GetGMPVideoDecoderCallback;
typedef GMPVideoGetterCallback<GMPVideoEncoderProxy> GetGMPVideoEncoderCallback;
class GetNodeIdCallback
{
public:
  MOZ_COUNTED_DEFAULT_CTOR(GetNodeIdCallback)
  MOZ_COUNTED_DTOR_VIRTUAL(GetNodeIdCallback)
  virtual void Done(nsresult aResult, const nsACString& aNodeId) = 0;
};

/* starting interface:    mozIGeckoMediaPluginService */
#define MOZIGECKOMEDIAPLUGINSERVICE_IID_STR "44d362ae-937a-4803-bee6-f2512a0149d1"

#define MOZIGECKOMEDIAPLUGINSERVICE_IID \
  {0x44d362ae, 0x937a, 0x4803, \
    { 0xbe, 0xe6, 0xf2, 0x51, 0x2a, 0x01, 0x49, 0xd1 }}

class NS_NO_VTABLE mozIGeckoMediaPluginService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIGECKOMEDIAPLUGINSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIGeckoMediaPluginService;

  /* readonly attribute nsIThread thread; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetThread(nsIThread **aThread) = 0;

  /* void RunPluginCrashCallbacks (in unsigned long pluginId, in ACString pluginName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RunPluginCrashCallbacks(uint32_t pluginId, const nsACString& pluginName) = 0;

  /* [noscript] boolean hasPluginForAPI (in ACString api, in TagArray tags); */
  NS_IMETHOD HasPluginForAPI(const nsACString& api, nsTArray<nsCString> * tags, bool *_retval) = 0;

  /* [noscript] void getGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoDecoderCallback callback); */
  NS_IMETHOD GetGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback) = 0;

  /* [noscript] void getDecryptingGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPVideoDecoderCallback callback, in uint32_t decryptorId); */
  NS_IMETHOD GetDecryptingGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback, uint32_t decryptorId) = 0;

  /* [noscript] void getGMPVideoEncoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoEncoderCallback callback); */
  NS_IMETHOD GetGMPVideoEncoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoEncoderCallback>&& callback) = 0;

  /* [noscript] void getNodeId (in AString origin, in AString topLevelOrigin, in AString gmpName, in GetNodeIdCallback callback); */
  NS_IMETHOD GetNodeId(const nsAString& origin, const nsAString& topLevelOrigin, const nsAString& gmpName, mozilla::UniquePtr<GetNodeIdCallback>&& callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIGeckoMediaPluginService, MOZIGECKOMEDIAPLUGINSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIGECKOMEDIAPLUGINSERVICE \
  NS_IMETHOD GetThread(nsIThread **aThread) override; \
  NS_IMETHOD RunPluginCrashCallbacks(uint32_t pluginId, const nsACString& pluginName) override; \
  NS_IMETHOD HasPluginForAPI(const nsACString& api, nsTArray<nsCString> * tags, bool *_retval) override; \
  NS_IMETHOD GetGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback) override; \
  NS_IMETHOD GetDecryptingGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback, uint32_t decryptorId) override; \
  NS_IMETHOD GetGMPVideoEncoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoEncoderCallback>&& callback) override; \
  NS_IMETHOD GetNodeId(const nsAString& origin, const nsAString& topLevelOrigin, const nsAString& gmpName, mozilla::UniquePtr<GetNodeIdCallback>&& callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIGECKOMEDIAPLUGINSERVICE \
  nsresult GetThread(nsIThread **aThread); \
  nsresult RunPluginCrashCallbacks(uint32_t pluginId, const nsACString& pluginName); \
  nsresult HasPluginForAPI(const nsACString& api, nsTArray<nsCString> * tags, bool *_retval); \
  nsresult GetGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback); \
  nsresult GetDecryptingGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback, uint32_t decryptorId); \
  nsresult GetGMPVideoEncoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoEncoderCallback>&& callback); \
  nsresult GetNodeId(const nsAString& origin, const nsAString& topLevelOrigin, const nsAString& gmpName, mozilla::UniquePtr<GetNodeIdCallback>&& callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIGECKOMEDIAPLUGINSERVICE(_to) \
  NS_IMETHOD GetThread(nsIThread **aThread) override { return _to GetThread(aThread); } \
  NS_IMETHOD RunPluginCrashCallbacks(uint32_t pluginId, const nsACString& pluginName) override { return _to RunPluginCrashCallbacks(pluginId, pluginName); } \
  NS_IMETHOD HasPluginForAPI(const nsACString& api, nsTArray<nsCString> * tags, bool *_retval) override { return _to HasPluginForAPI(api, tags, _retval); } \
  NS_IMETHOD GetGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback) override { return _to GetGMPVideoDecoder(helper, tags, nodeId, callback); } \
  NS_IMETHOD GetDecryptingGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback, uint32_t decryptorId) override { return _to GetDecryptingGMPVideoDecoder(helper, tags, nodeId, callback, decryptorId); } \
  NS_IMETHOD GetGMPVideoEncoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoEncoderCallback>&& callback) override { return _to GetGMPVideoEncoder(helper, tags, nodeId, callback); } \
  NS_IMETHOD GetNodeId(const nsAString& origin, const nsAString& topLevelOrigin, const nsAString& gmpName, mozilla::UniquePtr<GetNodeIdCallback>&& callback) override { return _to GetNodeId(origin, topLevelOrigin, gmpName, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIGECKOMEDIAPLUGINSERVICE(_to) \
  NS_IMETHOD GetThread(nsIThread **aThread) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetThread(aThread); } \
  NS_IMETHOD RunPluginCrashCallbacks(uint32_t pluginId, const nsACString& pluginName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunPluginCrashCallbacks(pluginId, pluginName); } \
  NS_IMETHOD HasPluginForAPI(const nsACString& api, nsTArray<nsCString> * tags, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasPluginForAPI(api, tags, _retval); } \
  NS_IMETHOD GetGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGMPVideoDecoder(helper, tags, nodeId, callback); } \
  NS_IMETHOD GetDecryptingGMPVideoDecoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoDecoderCallback>&& callback, uint32_t decryptorId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDecryptingGMPVideoDecoder(helper, tags, nodeId, callback, decryptorId); } \
  NS_IMETHOD GetGMPVideoEncoder(mozilla::GMPCrashHelper* helper, nsTArray<nsCString> * tags, const nsACString& nodeId, mozilla::UniquePtr<GetGMPVideoEncoderCallback>&& callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGMPVideoEncoder(helper, tags, nodeId, callback); } \
  NS_IMETHOD GetNodeId(const nsAString& origin, const nsAString& topLevelOrigin, const nsAString& gmpName, mozilla::UniquePtr<GetNodeIdCallback>&& callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNodeId(origin, topLevelOrigin, gmpName, callback); } 


#endif /* __gen_mozIGeckoMediaPluginService_h__ */
