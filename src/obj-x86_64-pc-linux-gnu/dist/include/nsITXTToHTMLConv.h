/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsITXTToHTMLConv.idl
 */

#ifndef __gen_nsITXTToHTMLConv_h__
#define __gen_nsITXTToHTMLConv_h__


#ifndef __gen_nsIStreamConverter_h__
#include "nsIStreamConverter.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsITXTToHTMLConv */
#define NS_ITXTTOHTMLCONV_IID_STR "933355f6-1dd2-11b2-a9b0-d335b9e35983"

#define NS_ITXTTOHTMLCONV_IID \
  {0x933355f6, 0x1dd2, 0x11b2, \
    { 0xa9, 0xb0, 0xd3, 0x35, 0xb9, 0xe3, 0x59, 0x83 }}

class NS_NO_VTABLE nsITXTToHTMLConv : public nsIStreamConverter {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITXTTOHTMLCONV_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITXTToHTMLConv;

  /* void setTitle (in wstring text); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTitle(const char16_t * text) = 0;

  /* void preFormatHTML (in boolean value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreFormatHTML(bool value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITXTToHTMLConv, NS_ITXTTOHTMLCONV_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITXTTOHTMLCONV \
  NS_IMETHOD SetTitle(const char16_t * text) override; \
  NS_IMETHOD PreFormatHTML(bool value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITXTTOHTMLCONV \
  nsresult SetTitle(const char16_t * text); \
  nsresult PreFormatHTML(bool value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITXTTOHTMLCONV(_to) \
  NS_IMETHOD SetTitle(const char16_t * text) override { return _to SetTitle(text); } \
  NS_IMETHOD PreFormatHTML(bool value) override { return _to PreFormatHTML(value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITXTTOHTMLCONV(_to) \
  NS_IMETHOD SetTitle(const char16_t * text) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTitle(text); } \
  NS_IMETHOD PreFormatHTML(bool value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreFormatHTML(value); } 


#endif /* __gen_nsITXTToHTMLConv_h__ */
