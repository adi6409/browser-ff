/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgressParams.idl
 */

#ifndef __gen_nsIPrintProgressParams_h__
#define __gen_nsIPrintProgressParams_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrintProgressParams */
#define NS_IPRINTPROGRESSPARAMS_IID_STR "ca89b55b-6faf-4051-9645-1c03ef5108f8"

#define NS_IPRINTPROGRESSPARAMS_IID \
  {0xca89b55b, 0x6faf, 0x4051, \
    { 0x96, 0x45, 0x1c, 0x03, 0xef, 0x51, 0x08, 0xf8 }}

class NS_NO_VTABLE nsIPrintProgressParams : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTPROGRESSPARAMS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrintProgressParams;

  /* attribute AString docTitle; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocTitle(nsAString& aDocTitle) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocTitle(const nsAString& aDocTitle) = 0;

  /* attribute AString docURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocURL(nsAString& aDocURL) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocURL(const nsAString& aDocURL) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrintProgressParams, NS_IPRINTPROGRESSPARAMS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTPROGRESSPARAMS \
  NS_IMETHOD GetDocTitle(nsAString& aDocTitle) override; \
  NS_IMETHOD SetDocTitle(const nsAString& aDocTitle) override; \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override; \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTPROGRESSPARAMS \
  nsresult GetDocTitle(nsAString& aDocTitle); \
  nsresult SetDocTitle(const nsAString& aDocTitle); \
  nsresult GetDocURL(nsAString& aDocURL); \
  nsresult SetDocURL(const nsAString& aDocURL); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTPROGRESSPARAMS(_to) \
  NS_IMETHOD GetDocTitle(nsAString& aDocTitle) override { return _to GetDocTitle(aDocTitle); } \
  NS_IMETHOD SetDocTitle(const nsAString& aDocTitle) override { return _to SetDocTitle(aDocTitle); } \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override { return _to GetDocURL(aDocURL); } \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override { return _to SetDocURL(aDocURL); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTPROGRESSPARAMS(_to) \
  NS_IMETHOD GetDocTitle(nsAString& aDocTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocTitle(aDocTitle); } \
  NS_IMETHOD SetDocTitle(const nsAString& aDocTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocTitle(aDocTitle); } \
  NS_IMETHOD GetDocURL(nsAString& aDocURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocURL(aDocURL); } \
  NS_IMETHOD SetDocURL(const nsAString& aDocURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocURL(aDocURL); } 


#endif /* __gen_nsIPrintProgressParams_h__ */
