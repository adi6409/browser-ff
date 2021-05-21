/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolHandler.idl
 */

#ifndef __gen_nsIProtocolHandler_h__
#define __gen_nsIProtocolHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsCOMPtr.h"
/**
 * Protocol handlers are registered with XPCOM under the following CONTRACTID prefix:
 */
#define NS_NETWORK_PROTOCOL_CONTRACTID_PREFIX "@mozilla.org/network/protocol;1?name="
/**
 * For example, "@mozilla.org/network/protocol;1?name=http"
 */
#if defined(MOZ_THUNDERBIRD) || defined(MOZ_SUITE)
#define IS_ORIGIN_IS_FULL_SPEC_DEFINED 1
#endif
class nsIURI; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsILoadInfo; /* forward declaration */


/* starting interface:    nsIProtocolHandlerWithDynamicFlags */
#define NS_IPROTOCOLHANDLERWITHDYNAMICFLAGS_IID_STR "65a8e823-0591-4fc0-a56a-03265e0a4ce8"

#define NS_IPROTOCOLHANDLERWITHDYNAMICFLAGS_IID \
  {0x65a8e823, 0x0591, 0x4fc0, \
    { 0xa5, 0x6a, 0x03, 0x26, 0x5e, 0x0a, 0x4c, 0xe8 }}

class NS_NO_VTABLE nsIProtocolHandlerWithDynamicFlags : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLHANDLERWITHDYNAMICFLAGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolHandlerWithDynamicFlags;

  /* unsigned long getFlagsForURI (in nsIURI aURI); */
  NS_IMETHOD GetFlagsForURI(nsIURI *aURI, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolHandlerWithDynamicFlags, NS_IPROTOCOLHANDLERWITHDYNAMICFLAGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLHANDLERWITHDYNAMICFLAGS \
  NS_IMETHOD GetFlagsForURI(nsIURI *aURI, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLHANDLERWITHDYNAMICFLAGS \
  nsresult GetFlagsForURI(nsIURI *aURI, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLHANDLERWITHDYNAMICFLAGS(_to) \
  NS_IMETHOD GetFlagsForURI(nsIURI *aURI, uint32_t *_retval) override { return _to GetFlagsForURI(aURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLHANDLERWITHDYNAMICFLAGS(_to) \
  NS_IMETHOD GetFlagsForURI(nsIURI *aURI, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlagsForURI(aURI, _retval); } 


/* starting interface:    nsIProtocolHandler */
#define NS_IPROTOCOLHANDLER_IID_STR "a87210e6-7c8c-41f7-864d-df809015193e"

#define NS_IPROTOCOLHANDLER_IID \
  {0xa87210e6, 0x7c8c, 0x41f7, \
    { 0x86, 0x4d, 0xdf, 0x80, 0x90, 0x15, 0x19, 0x3e }}

class nsIProtocolHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProtocolHandler;

  /* readonly attribute ACString scheme; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScheme(nsACString& aScheme) = 0;

  /* readonly attribute long defaultPort; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultPort(int32_t *aDefaultPort) = 0;

  /* readonly attribute unsigned long protocolFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProtocolFlags(uint32_t *aProtocolFlags) = 0;

   // Helper method to get the protocol flags in the right way.
  nsresult DoGetProtocolFlags(nsIURI* aURI, uint32_t* aFlags)
  {
      nsCOMPtr<nsIProtocolHandlerWithDynamicFlags> dh = do_QueryInterface(this);
      nsresult rv = dh ? dh->GetFlagsForURI(aURI, aFlags) : GetProtocolFlags(aFlags);
      if (NS_SUCCEEDED(rv)) {
#if !IS_ORIGIN_IS_FULL_SPEC_DEFINED
        MOZ_RELEASE_ASSERT(!(*aFlags & nsIProtocolHandler::ORIGIN_IS_FULL_SPEC),
                           "ORIGIN_IS_FULL_SPEC is unsupported but used");
#endif
      }
      return rv;
  }
  /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewChannel(nsIURI *aURI, nsILoadInfo *aLoadinfo, nsIChannel **_retval) = 0;

  /* boolean allowPort (in long port, in string scheme); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowPort(int32_t port, const char * scheme, bool *_retval) = 0;

  enum {
    URI_STD = 0U,
    URI_NORELATIVE = 1U,
    URI_NOAUTH = 2U,
    ALLOWS_PROXY = 4U,
    ALLOWS_PROXY_HTTP = 8U,
    URI_INHERITS_SECURITY_CONTEXT = 16U,
    URI_FORBIDS_AUTOMATIC_DOCUMENT_REPLACEMENT = 32U,
    URI_LOADABLE_BY_ANYONE = 64U,
    URI_DANGEROUS_TO_LOAD = 128U,
    URI_IS_UI_RESOURCE = 256U,
    URI_IS_LOCAL_FILE = 512U,
    URI_LOADABLE_BY_SUBSUMERS = 1024U,
    URI_DOES_NOT_RETURN_DATA = 2048U,
    URI_IS_LOCAL_RESOURCE = 4096U,
    URI_OPENING_EXECUTES_SCRIPT = 8192U,
    URI_NON_PERSISTABLE = 16384U,
    URI_CROSS_ORIGIN_NEEDS_WEBAPPS_PERM = 32768U,
    URI_SYNC_LOAD_IS_OK = 65536U,
    URI_IS_POTENTIALLY_TRUSTWORTHY = 131072U,
    URI_FETCHABLE_BY_ANYONE = 262144U,
    ORIGIN_IS_FULL_SPEC = 524288U,
    URI_SCHEME_NOT_SELF_LINKABLE = 1048576U,
    URI_LOADABLE_BY_EXTENSIONS = 2097152U,
    URI_DISALLOW_IN_PRIVATE_CONTEXT = 4194304U,
    URI_FORBIDS_COOKIE_ACCESS = 8388608U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProtocolHandler, NS_IPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROTOCOLHANDLER \
  NS_IMETHOD GetScheme(nsACString& aScheme) override; \
  NS_IMETHOD GetDefaultPort(int32_t *aDefaultPort) override; \
  NS_IMETHOD GetProtocolFlags(uint32_t *aProtocolFlags) override; \
  NS_IMETHOD NewChannel(nsIURI *aURI, nsILoadInfo *aLoadinfo, nsIChannel **_retval) override; \
  NS_IMETHOD AllowPort(int32_t port, const char * scheme, bool *_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROTOCOLHANDLER \
  nsresult GetScheme(nsACString& aScheme); \
  nsresult GetDefaultPort(int32_t *aDefaultPort); \
  nsresult GetProtocolFlags(uint32_t *aProtocolFlags); \
  nsresult NewChannel(nsIURI *aURI, nsILoadInfo *aLoadinfo, nsIChannel **_retval); \
  nsresult AllowPort(int32_t port, const char * scheme, bool *_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROTOCOLHANDLER(_to) \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return _to GetScheme(aScheme); } \
  NS_IMETHOD GetDefaultPort(int32_t *aDefaultPort) override { return _to GetDefaultPort(aDefaultPort); } \
  NS_IMETHOD GetProtocolFlags(uint32_t *aProtocolFlags) override { return _to GetProtocolFlags(aProtocolFlags); } \
  NS_IMETHOD NewChannel(nsIURI *aURI, nsILoadInfo *aLoadinfo, nsIChannel **_retval) override { return _to NewChannel(aURI, aLoadinfo, _retval); } \
  NS_IMETHOD AllowPort(int32_t port, const char * scheme, bool *_retval) override { return _to AllowPort(port, scheme, _retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROTOCOLHANDLER(_to) \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScheme(aScheme); } \
  NS_IMETHOD GetDefaultPort(int32_t *aDefaultPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultPort(aDefaultPort); } \
  NS_IMETHOD GetProtocolFlags(uint32_t *aProtocolFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolFlags(aProtocolFlags); } \
  NS_IMETHOD NewChannel(nsIURI *aURI, nsILoadInfo *aLoadinfo, nsIChannel **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewChannel(aURI, aLoadinfo, _retval); } \
  NS_IMETHOD AllowPort(int32_t port, const char * scheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowPort(port, scheme, _retval); } \


#endif /* __gen_nsIProtocolHandler_h__ */
