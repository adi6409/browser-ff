/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsITextToSubURI.idl
 */

#ifndef __gen_nsITextToSubURI_h__
#define __gen_nsITextToSubURI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
// {8B042E22-6F87-11d3-B3C8-00805F8A6670}
#define NS_TEXTTOSUBURI_CID { 0x8b042e22, 0x6f87, 0x11d3, { 0xb3, 0xc8, 0x0, 0x80, 0x5f, 0x8a, 0x66, 0x70 } }
#define NS_ITEXTTOSUBURI_CONTRACTID "@mozilla.org/intl/texttosuburi;1"

/* starting interface:    nsITextToSubURI */
#define NS_ITEXTTOSUBURI_IID_STR "8b042e24-6f87-11d3-b3c8-00805f8a6670"

#define NS_ITEXTTOSUBURI_IID \
  {0x8b042e24, 0x6f87, 0x11d3, \
    { 0xb3, 0xc8, 0x00, 0x80, 0x5f, 0x8a, 0x66, 0x70 }}

class NS_NO_VTABLE nsITextToSubURI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITEXTTOSUBURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITextToSubURI;

  /* ACString ConvertAndEscape (in ACString charset, in AString text); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertAndEscape(const nsACString& charset, const nsAString& text, nsACString& _retval) = 0;

  /* AString UnEscapeAndConvert (in ACString charset, in ACString text); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnEscapeAndConvert(const nsACString& charset, const nsACString& text, nsAString& _retval) = 0;

  /* AString unEscapeURIForUI (in AUTF8String aURIFragment); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnEscapeURIForUI(const nsACString& aURIFragment, nsAString& _retval) = 0;

  /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnEscapeNonAsciiURI(const nsACString& aCharset, const nsACString& aURIFragment, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITextToSubURI, NS_ITEXTTOSUBURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITEXTTOSUBURI \
  NS_IMETHOD ConvertAndEscape(const nsACString& charset, const nsAString& text, nsACString& _retval) override; \
  NS_IMETHOD UnEscapeAndConvert(const nsACString& charset, const nsACString& text, nsAString& _retval) override; \
  NS_IMETHOD UnEscapeURIForUI(const nsACString& aURIFragment, nsAString& _retval) override; \
  NS_IMETHOD UnEscapeNonAsciiURI(const nsACString& aCharset, const nsACString& aURIFragment, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITEXTTOSUBURI \
  nsresult ConvertAndEscape(const nsACString& charset, const nsAString& text, nsACString& _retval); \
  nsresult UnEscapeAndConvert(const nsACString& charset, const nsACString& text, nsAString& _retval); \
  nsresult UnEscapeURIForUI(const nsACString& aURIFragment, nsAString& _retval); \
  nsresult UnEscapeNonAsciiURI(const nsACString& aCharset, const nsACString& aURIFragment, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITEXTTOSUBURI(_to) \
  NS_IMETHOD ConvertAndEscape(const nsACString& charset, const nsAString& text, nsACString& _retval) override { return _to ConvertAndEscape(charset, text, _retval); } \
  NS_IMETHOD UnEscapeAndConvert(const nsACString& charset, const nsACString& text, nsAString& _retval) override { return _to UnEscapeAndConvert(charset, text, _retval); } \
  NS_IMETHOD UnEscapeURIForUI(const nsACString& aURIFragment, nsAString& _retval) override { return _to UnEscapeURIForUI(aURIFragment, _retval); } \
  NS_IMETHOD UnEscapeNonAsciiURI(const nsACString& aCharset, const nsACString& aURIFragment, nsAString& _retval) override { return _to UnEscapeNonAsciiURI(aCharset, aURIFragment, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITEXTTOSUBURI(_to) \
  NS_IMETHOD ConvertAndEscape(const nsACString& charset, const nsAString& text, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertAndEscape(charset, text, _retval); } \
  NS_IMETHOD UnEscapeAndConvert(const nsACString& charset, const nsACString& text, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnEscapeAndConvert(charset, text, _retval); } \
  NS_IMETHOD UnEscapeURIForUI(const nsACString& aURIFragment, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnEscapeURIForUI(aURIFragment, _retval); } \
  NS_IMETHOD UnEscapeNonAsciiURI(const nsACString& aCharset, const nsACString& aURIFragment, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnEscapeNonAsciiURI(aCharset, aURIFragment, _retval); } 


#endif /* __gen_nsITextToSubURI_h__ */
