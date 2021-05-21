/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptProvider.idl
 */

#ifndef __gen_nsIAuthPromptProvider_h__
#define __gen_nsIAuthPromptProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAuthPromptProvider */
#define NS_IAUTHPROMPTPROVIDER_IID_STR "bd9dc0fa-68ce-47d0-8859-6418c2ae8576"

#define NS_IAUTHPROMPTPROVIDER_IID \
  {0xbd9dc0fa, 0x68ce, 0x47d0, \
    { 0x88, 0x59, 0x64, 0x18, 0xc2, 0xae, 0x85, 0x76 }}

class NS_NO_VTABLE nsIAuthPromptProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTHPROMPTPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAuthPromptProvider;

  enum {
    PROMPT_NORMAL = 0U,
    PROMPT_PROXY = 1U
  };

  /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAuthPrompt(uint32_t aPromptReason, const nsIID & iid, void * * result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAuthPromptProvider, NS_IAUTHPROMPTPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTHPROMPTPROVIDER \
  NS_IMETHOD GetAuthPrompt(uint32_t aPromptReason, const nsIID & iid, void * * result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTHPROMPTPROVIDER \
  nsresult GetAuthPrompt(uint32_t aPromptReason, const nsIID & iid, void * * result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTHPROMPTPROVIDER(_to) \
  NS_IMETHOD GetAuthPrompt(uint32_t aPromptReason, const nsIID & iid, void * * result) override { return _to GetAuthPrompt(aPromptReason, iid, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTHPROMPTPROVIDER(_to) \
  NS_IMETHOD GetAuthPrompt(uint32_t aPromptReason, const nsIID & iid, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAuthPrompt(aPromptReason, iid, result); } 


#endif /* __gen_nsIAuthPromptProvider_h__ */
