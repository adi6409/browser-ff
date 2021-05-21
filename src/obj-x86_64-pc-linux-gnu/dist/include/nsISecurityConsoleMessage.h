/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsISecurityConsoleMessage.idl
 */

#ifndef __gen_nsISecurityConsoleMessage_h__
#define __gen_nsISecurityConsoleMessage_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISecurityConsoleMessage */
#define NS_ISECURITYCONSOLEMESSAGE_IID_STR "fe9fc9b6-dde2-11e2-a8f1-0a326188709b"

#define NS_ISECURITYCONSOLEMESSAGE_IID \
  {0xfe9fc9b6, 0xdde2, 0x11e2, \
    { 0xa8, 0xf1, 0x0a, 0x32, 0x61, 0x88, 0x70, 0x9b }}

class NS_NO_VTABLE nsISecurityConsoleMessage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISECURITYCONSOLEMESSAGE_IID)

  /* attribute AString tag; */
  NS_IMETHOD GetTag(nsAString& aTag) = 0;
  NS_IMETHOD SetTag(const nsAString& aTag) = 0;

  /* attribute AString category; */
  NS_IMETHOD GetCategory(nsAString& aCategory) = 0;
  NS_IMETHOD SetCategory(const nsAString& aCategory) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISecurityConsoleMessage, NS_ISECURITYCONSOLEMESSAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISECURITYCONSOLEMESSAGE \
  NS_IMETHOD GetTag(nsAString& aTag) override; \
  NS_IMETHOD SetTag(const nsAString& aTag) override; \
  NS_IMETHOD GetCategory(nsAString& aCategory) override; \
  NS_IMETHOD SetCategory(const nsAString& aCategory) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISECURITYCONSOLEMESSAGE \
  nsresult GetTag(nsAString& aTag); \
  nsresult SetTag(const nsAString& aTag); \
  nsresult GetCategory(nsAString& aCategory); \
  nsresult SetCategory(const nsAString& aCategory); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISECURITYCONSOLEMESSAGE(_to) \
  NS_IMETHOD GetTag(nsAString& aTag) override { return _to GetTag(aTag); } \
  NS_IMETHOD SetTag(const nsAString& aTag) override { return _to SetTag(aTag); } \
  NS_IMETHOD GetCategory(nsAString& aCategory) override { return _to GetCategory(aCategory); } \
  NS_IMETHOD SetCategory(const nsAString& aCategory) override { return _to SetCategory(aCategory); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISECURITYCONSOLEMESSAGE(_to) \
  NS_IMETHOD GetTag(nsAString& aTag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTag(aTag); } \
  NS_IMETHOD SetTag(const nsAString& aTag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTag(aTag); } \
  NS_IMETHOD GetCategory(nsAString& aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCategory(aCategory); } \
  NS_IMETHOD SetCategory(const nsAString& aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCategory(aCategory); } 

#define NS_SECURITY_CONSOLE_MESSAGE_CONTRACTID "@mozilla.org/securityconsole/message;1"

#endif /* __gen_nsISecurityConsoleMessage_h__ */
