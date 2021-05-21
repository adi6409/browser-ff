/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebPageDescriptor.idl
 */

#ifndef __gen_nsIWebPageDescriptor_h__
#define __gen_nsIWebPageDescriptor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDocShell; /* forward declaration */


/* starting interface:    nsIWebPageDescriptor */
#define NS_IWEBPAGEDESCRIPTOR_IID_STR "6f30b676-3710-4c2c-80b1-0395fb26516e"

#define NS_IWEBPAGEDESCRIPTOR_IID \
  {0x6f30b676, 0x3710, 0x4c2c, \
    { 0x80, 0xb1, 0x03, 0x95, 0xfb, 0x26, 0x51, 0x6e }}

class NS_NO_VTABLE nsIWebPageDescriptor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBPAGEDESCRIPTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebPageDescriptor;

  /* void loadPageAsViewSource (in nsIDocShell otherDocShell, in AString aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadPageAsViewSource(nsIDocShell *otherDocShell, const nsAString& aURL) = 0;

  /* readonly attribute nsISupports currentDescriptor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentDescriptor(nsISupports **aCurrentDescriptor) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebPageDescriptor, NS_IWEBPAGEDESCRIPTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBPAGEDESCRIPTOR \
  NS_IMETHOD LoadPageAsViewSource(nsIDocShell *otherDocShell, const nsAString& aURL) override; \
  NS_IMETHOD GetCurrentDescriptor(nsISupports **aCurrentDescriptor) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBPAGEDESCRIPTOR \
  nsresult LoadPageAsViewSource(nsIDocShell *otherDocShell, const nsAString& aURL); \
  nsresult GetCurrentDescriptor(nsISupports **aCurrentDescriptor); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBPAGEDESCRIPTOR(_to) \
  NS_IMETHOD LoadPageAsViewSource(nsIDocShell *otherDocShell, const nsAString& aURL) override { return _to LoadPageAsViewSource(otherDocShell, aURL); } \
  NS_IMETHOD GetCurrentDescriptor(nsISupports **aCurrentDescriptor) override { return _to GetCurrentDescriptor(aCurrentDescriptor); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBPAGEDESCRIPTOR(_to) \
  NS_IMETHOD LoadPageAsViewSource(nsIDocShell *otherDocShell, const nsAString& aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadPageAsViewSource(otherDocShell, aURL); } \
  NS_IMETHOD GetCurrentDescriptor(nsISupports **aCurrentDescriptor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentDescriptor(aCurrentDescriptor); } 


#endif /* __gen_nsIWebPageDescriptor_h__ */
