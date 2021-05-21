/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISharePicker.idl
 */

#ifndef __gen_nsISharePicker_h__
#define __gen_nsISharePicker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsISharePicker */
#define NS_ISHAREPICKER_IID_STR "1201d357-8417-4926-a694-e6408fbedcf8"

#define NS_ISHAREPICKER_IID \
  {0x1201d357, 0x8417, 0x4926, \
    { 0xa6, 0x94, 0xe6, 0x40, 0x8f, 0xbe, 0xdc, 0xf8 }}

class NS_NO_VTABLE nsISharePicker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHAREPICKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISharePicker;

  /* void init (in mozIDOMWindowProxy openerWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(mozIDOMWindowProxy *openerWindow) = 0;

  /* readonly attribute mozIDOMWindowProxy openerWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOpenerWindow(mozIDOMWindowProxy **aOpenerWindow) = 0;

  /* Promise share (in AUTF8String title, in AUTF8String text, in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Share(const nsACString& title, const nsACString& text, nsIURI *url, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISharePicker, NS_ISHAREPICKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHAREPICKER \
  NS_IMETHOD Init(mozIDOMWindowProxy *openerWindow) override; \
  NS_IMETHOD GetOpenerWindow(mozIDOMWindowProxy **aOpenerWindow) override; \
  NS_IMETHOD Share(const nsACString& title, const nsACString& text, nsIURI *url, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHAREPICKER \
  nsresult Init(mozIDOMWindowProxy *openerWindow); \
  nsresult GetOpenerWindow(mozIDOMWindowProxy **aOpenerWindow); \
  nsresult Share(const nsACString& title, const nsACString& text, nsIURI *url, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHAREPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *openerWindow) override { return _to Init(openerWindow); } \
  NS_IMETHOD GetOpenerWindow(mozIDOMWindowProxy **aOpenerWindow) override { return _to GetOpenerWindow(aOpenerWindow); } \
  NS_IMETHOD Share(const nsACString& title, const nsACString& text, nsIURI *url, ::mozilla::dom::Promise * * _retval) override { return _to Share(title, text, url, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHAREPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *openerWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(openerWindow); } \
  NS_IMETHOD GetOpenerWindow(mozIDOMWindowProxy **aOpenerWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpenerWindow(aOpenerWindow); } \
  NS_IMETHOD Share(const nsACString& title, const nsACString& text, nsIURI *url, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Share(title, text, url, _retval); } 


#endif /* __gen_nsISharePicker_h__ */
