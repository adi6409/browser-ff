/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIE10SUtils.idl
 */

#ifndef __gen_nsIE10SUtils_h__
#define __gen_nsIE10SUtils_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIE10SUtils */
#define NS_IE10SUTILS_IID_STR "1e18680e-052d-4509-a17e-678f5c495e02"

#define NS_IE10SUTILS_IID \
  {0x1e18680e, 0x052d, 0x4509, \
    { 0xa1, 0x7e, 0x67, 0x8f, 0x5c, 0x49, 0x5e, 0x02 }}

class NS_NO_VTABLE nsIE10SUtils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IE10SUTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIE10SUtils;

  enum RemoteWorkerType : uint8_t {
    REMOTE_WORKER_TYPE_SHARED = 0,
    REMOTE_WORKER_TYPE_SERVICE = 1,
  };

  /* AUTF8String getRemoteTypeForPrincipal (in nsIPrincipal aPrincipal, in nsIURI aChannelOriginalURI, in boolean aMultiProcess, in boolean aRemoteSubframes, in AUTF8String aPreferredRemoteType, in nsIPrincipal aCurrentPrincipal, in boolean aIsSubframe); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteTypeForPrincipal(nsIPrincipal *aPrincipal, nsIURI *aChannelOriginalURI, bool aMultiProcess, bool aRemoteSubframes, const nsACString& aPreferredRemoteType, nsIPrincipal *aCurrentPrincipal, bool aIsSubframe, nsACString& _retval) = 0;

  /* AUTF8String getRemoteTypeForWorkerPrincipal (in nsIPrincipal aPrincipal, in nsIE10SUtils_RemoteWorkerType aWorkerType, in boolean aIsMultiProcess, in boolean aIsFission, in AUTF8String aPreferredRemoteType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteTypeForWorkerPrincipal(nsIPrincipal *aPrincipal, nsIE10SUtils::RemoteWorkerType aWorkerType, bool aIsMultiProcess, bool aIsFission, const nsACString& aPreferredRemoteType, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIE10SUtils, NS_IE10SUTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIE10SUTILS \
  NS_IMETHOD GetRemoteTypeForPrincipal(nsIPrincipal *aPrincipal, nsIURI *aChannelOriginalURI, bool aMultiProcess, bool aRemoteSubframes, const nsACString& aPreferredRemoteType, nsIPrincipal *aCurrentPrincipal, bool aIsSubframe, nsACString& _retval) override; \
  NS_IMETHOD GetRemoteTypeForWorkerPrincipal(nsIPrincipal *aPrincipal, nsIE10SUtils::RemoteWorkerType aWorkerType, bool aIsMultiProcess, bool aIsFission, const nsACString& aPreferredRemoteType, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIE10SUTILS \
  nsresult GetRemoteTypeForPrincipal(nsIPrincipal *aPrincipal, nsIURI *aChannelOriginalURI, bool aMultiProcess, bool aRemoteSubframes, const nsACString& aPreferredRemoteType, nsIPrincipal *aCurrentPrincipal, bool aIsSubframe, nsACString& _retval); \
  nsresult GetRemoteTypeForWorkerPrincipal(nsIPrincipal *aPrincipal, nsIE10SUtils::RemoteWorkerType aWorkerType, bool aIsMultiProcess, bool aIsFission, const nsACString& aPreferredRemoteType, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIE10SUTILS(_to) \
  NS_IMETHOD GetRemoteTypeForPrincipal(nsIPrincipal *aPrincipal, nsIURI *aChannelOriginalURI, bool aMultiProcess, bool aRemoteSubframes, const nsACString& aPreferredRemoteType, nsIPrincipal *aCurrentPrincipal, bool aIsSubframe, nsACString& _retval) override { return _to GetRemoteTypeForPrincipal(aPrincipal, aChannelOriginalURI, aMultiProcess, aRemoteSubframes, aPreferredRemoteType, aCurrentPrincipal, aIsSubframe, _retval); } \
  NS_IMETHOD GetRemoteTypeForWorkerPrincipal(nsIPrincipal *aPrincipal, nsIE10SUtils::RemoteWorkerType aWorkerType, bool aIsMultiProcess, bool aIsFission, const nsACString& aPreferredRemoteType, nsACString& _retval) override { return _to GetRemoteTypeForWorkerPrincipal(aPrincipal, aWorkerType, aIsMultiProcess, aIsFission, aPreferredRemoteType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIE10SUTILS(_to) \
  NS_IMETHOD GetRemoteTypeForPrincipal(nsIPrincipal *aPrincipal, nsIURI *aChannelOriginalURI, bool aMultiProcess, bool aRemoteSubframes, const nsACString& aPreferredRemoteType, nsIPrincipal *aCurrentPrincipal, bool aIsSubframe, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteTypeForPrincipal(aPrincipal, aChannelOriginalURI, aMultiProcess, aRemoteSubframes, aPreferredRemoteType, aCurrentPrincipal, aIsSubframe, _retval); } \
  NS_IMETHOD GetRemoteTypeForWorkerPrincipal(nsIPrincipal *aPrincipal, nsIE10SUtils::RemoteWorkerType aWorkerType, bool aIsMultiProcess, bool aIsFission, const nsACString& aPreferredRemoteType, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteTypeForWorkerPrincipal(aPrincipal, aWorkerType, aIsMultiProcess, aIsFission, aPreferredRemoteType, _retval); } 


#endif /* __gen_nsIE10SUtils_h__ */
