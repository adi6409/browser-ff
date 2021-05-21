/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome3.idl
 */

#ifndef __gen_nsIWebBrowserChrome3_h__
#define __gen_nsIWebBrowserChrome3_h__


#ifndef __gen_nsIWebBrowserChrome_h__
#include "nsIWebBrowserChrome.h"
#endif

#ifndef __gen_nsIURI_h__
#include "nsIURI.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDocShell; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIRunnable; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIContentSecurityPolicy; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIWebBrowserChrome3 */
#define NS_IWEBBROWSERCHROME3_IID_STR "542b6625-35a9-426a-8257-c12a345383b0"

#define NS_IWEBBROWSERCHROME3_IID \
  {0x542b6625, 0x35a9, 0x426a, \
    { 0x82, 0x57, 0xc1, 0x2a, 0x34, 0x53, 0x83, 0xb0 }}

class NS_NO_VTABLE nsIWebBrowserChrome3 : public nsIWebBrowserChrome {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERCHROME3_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserChrome3;

  /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) = 0;

  /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal, in nsIContentSecurityPolicy aCsp); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShouldLoadURI(nsIDocShell *aDocShell, nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, bool aHasPostData, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, bool *_retval) = 0;

  /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShouldLoadURIInThisProcess(nsIURI *aURI, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserChrome3, NS_IWEBBROWSERCHROME3_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERCHROME3 \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override; \
  NS_IMETHOD ShouldLoadURI(nsIDocShell *aDocShell, nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, bool aHasPostData, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, bool *_retval) override; \
  NS_IMETHOD ShouldLoadURIInThisProcess(nsIURI *aURI, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERCHROME3 \
  nsresult OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval); \
  nsresult ShouldLoadURI(nsIDocShell *aDocShell, nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, bool aHasPostData, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, bool *_retval); \
  nsresult ShouldLoadURIInThisProcess(nsIURI *aURI, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERCHROME3(_to) \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override { return _to OnBeforeLinkTraversal(originalTarget, linkURI, linkNode, isAppTab, _retval); } \
  NS_IMETHOD ShouldLoadURI(nsIDocShell *aDocShell, nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, bool aHasPostData, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, bool *_retval) override { return _to ShouldLoadURI(aDocShell, aURI, aReferrerInfo, aHasPostData, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD ShouldLoadURIInThisProcess(nsIURI *aURI, bool *_retval) override { return _to ShouldLoadURIInThisProcess(aURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERCHROME3(_to) \
  NS_IMETHOD OnBeforeLinkTraversal(const nsAString& originalTarget, nsIURI *linkURI, nsINode *linkNode, bool isAppTab, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBeforeLinkTraversal(originalTarget, linkURI, linkNode, isAppTab, _retval); } \
  NS_IMETHOD ShouldLoadURI(nsIDocShell *aDocShell, nsIURI *aURI, nsIReferrerInfo *aReferrerInfo, bool aHasPostData, nsIPrincipal *aTriggeringPrincipal, nsIContentSecurityPolicy *aCsp, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldLoadURI(aDocShell, aURI, aReferrerInfo, aHasPostData, aTriggeringPrincipal, aCsp, _retval); } \
  NS_IMETHOD ShouldLoadURIInThisProcess(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldLoadURIInThisProcess(aURI, _retval); } 


#endif /* __gen_nsIWebBrowserChrome3_h__ */
