/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webauthn/nsIU2FTokenManager.idl
 */

#ifndef __gen_nsIU2FTokenManager_h__
#define __gen_nsIU2FTokenManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIU2FTokenManager */
#define NS_IU2FTOKENMANAGER_IID_STR "745e1eac-e449-4342-bca1-ee0e6ead09fc"

#define NS_IU2FTOKENMANAGER_IID \
  {0x745e1eac, 0xe449, 0x4342, \
    { 0xbc, 0xa1, 0xee, 0x0e, 0x6e, 0xad, 0x09, 0xfc }}

class NS_NO_VTABLE nsIU2FTokenManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IU2FTOKENMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIU2FTokenManager;

  /* void resumeRegister (in uint64_t aTransactionID, in bool aForceNoneAttestation); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResumeRegister(uint64_t aTransactionID, bool aForceNoneAttestation) = 0;

  /* void cancel (in uint64_t aTransactionID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(uint64_t aTransactionID) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIU2FTokenManager, NS_IU2FTOKENMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIU2FTOKENMANAGER \
  NS_IMETHOD ResumeRegister(uint64_t aTransactionID, bool aForceNoneAttestation) override; \
  NS_IMETHOD Cancel(uint64_t aTransactionID) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIU2FTOKENMANAGER \
  nsresult ResumeRegister(uint64_t aTransactionID, bool aForceNoneAttestation); \
  nsresult Cancel(uint64_t aTransactionID); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIU2FTOKENMANAGER(_to) \
  NS_IMETHOD ResumeRegister(uint64_t aTransactionID, bool aForceNoneAttestation) override { return _to ResumeRegister(aTransactionID, aForceNoneAttestation); } \
  NS_IMETHOD Cancel(uint64_t aTransactionID) override { return _to Cancel(aTransactionID); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIU2FTOKENMANAGER(_to) \
  NS_IMETHOD ResumeRegister(uint64_t aTransactionID, bool aForceNoneAttestation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeRegister(aTransactionID, aForceNoneAttestation); } \
  NS_IMETHOD Cancel(uint64_t aTransactionID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(aTransactionID); } 


#endif /* __gen_nsIU2FTokenManager_h__ */
