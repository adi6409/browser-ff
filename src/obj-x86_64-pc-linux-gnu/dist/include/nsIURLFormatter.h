/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/urlformatter/nsIURLFormatter.idl
 */

#ifndef __gen_nsIURLFormatter_h__
#define __gen_nsIURLFormatter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIURLFormatter */
#define NS_IURLFORMATTER_IID_STR "4ab31d30-372d-11db-a98b-0800200c9a66"

#define NS_IURLFORMATTER_IID \
  {0x4ab31d30, 0x372d, 0x11db, \
    { 0xa9, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIURLFormatter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLFORMATTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURLFormatter;

  /* AString formatURL (in AString aFormat); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatURL(const nsAString& aFormat, nsAString& _retval) = 0;

  /* AString formatURLPref (in AString aPref); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatURLPref(const nsAString& aPref, nsAString& _retval) = 0;

  /* AString trimSensitiveURLs (in AString aMsg); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TrimSensitiveURLs(const nsAString& aMsg, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURLFormatter, NS_IURLFORMATTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLFORMATTER \
  NS_IMETHOD FormatURL(const nsAString& aFormat, nsAString& _retval) override; \
  NS_IMETHOD FormatURLPref(const nsAString& aPref, nsAString& _retval) override; \
  NS_IMETHOD TrimSensitiveURLs(const nsAString& aMsg, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLFORMATTER \
  nsresult FormatURL(const nsAString& aFormat, nsAString& _retval); \
  nsresult FormatURLPref(const nsAString& aPref, nsAString& _retval); \
  nsresult TrimSensitiveURLs(const nsAString& aMsg, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLFORMATTER(_to) \
  NS_IMETHOD FormatURL(const nsAString& aFormat, nsAString& _retval) override { return _to FormatURL(aFormat, _retval); } \
  NS_IMETHOD FormatURLPref(const nsAString& aPref, nsAString& _retval) override { return _to FormatURLPref(aPref, _retval); } \
  NS_IMETHOD TrimSensitiveURLs(const nsAString& aMsg, nsAString& _retval) override { return _to TrimSensitiveURLs(aMsg, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLFORMATTER(_to) \
  NS_IMETHOD FormatURL(const nsAString& aFormat, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatURL(aFormat, _retval); } \
  NS_IMETHOD FormatURLPref(const nsAString& aPref, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatURLPref(aPref, _retval); } \
  NS_IMETHOD TrimSensitiveURLs(const nsAString& aMsg, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TrimSensitiveURLs(aMsg, _retval); } 


#endif /* __gen_nsIURLFormatter_h__ */
