/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISecurityUITelemetry.idl
 */

#ifndef __gen_nsISecurityUITelemetry_h__
#define __gen_nsISecurityUITelemetry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISecurityUITelemetry */
#define NS_ISECURITYUITELEMETRY_IID_STR "5d1acf82-223a-46fb-a8f3-a1b16e2ceb04"

#define NS_ISECURITYUITELEMETRY_IID \
  {0x5d1acf82, 0x223a, 0x46fb, \
    { 0xa8, 0xf3, 0xa1, 0xb1, 0x6e, 0x2c, 0xeb, 0x04 }}

class NS_NO_VTABLE nsISecurityUITelemetry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISECURITYUITELEMETRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISecurityUITelemetry;

  enum {
    WARNING_ADDON_ASKING_PREVENTED = 1U,
    WARNING_ADDON_ASKING_PREVENTED_CLICK_THROUGH = 2U,
    WARNING_CONFIRM_ADDON_INSTALL = 3U,
    WARNING_CONFIRM_ADDON_INSTALL_CLICK_THROUGH = 4U,
    WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE = 9U,
    WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE_CLICK_THROUGH = 10U,
    WARNING_BAD_CERT_ADD_EXCEPTION_BASE = 30U,
    WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_UNTRUSTED = 1U,
    WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_DOMAIN = 2U,
    WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_TIME = 4U,
    WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_BASE = 38U,
    WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED = 1U,
    WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN = 2U,
    WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_TIME = 4U,
    WARNING_BAD_CERT_TOP_CLICK_VIEW_CERT = 71U,
    WARNING_BAD_CERT_TOP_DONT_REMEMBER_EXCEPTION = 72U,
    WARNING_BAD_CERT_TOP_ADD_EXCEPTION_BASE = 76U,
    WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_UNTRUSTED = 1U,
    WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_DOMAIN = 2U,
    WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_TIME = 4U,
    WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_BASE = 84U,
    WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED = 1U,
    WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN = 2U,
    WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_TIME = 4U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISecurityUITelemetry, NS_ISECURITYUITELEMETRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISECURITYUITELEMETRY \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISECURITYUITELEMETRY \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISECURITYUITELEMETRY(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISECURITYUITELEMETRY(_to) \


#endif /* __gen_nsISecurityUITelemetry_h__ */
