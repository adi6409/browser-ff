/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/IUrlClassifierUITelemetry.idl
 */

#ifndef __gen_IUrlClassifierUITelemetry_h__
#define __gen_IUrlClassifierUITelemetry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    IUrlClassifierUITelemetry */
#define IURLCLASSIFIERUITELEMETRY_IID_STR "a6c62ce5-3a95-41bb-b0f1-8cd4f4ca00e3"

#define IURLCLASSIFIERUITELEMETRY_IID \
  {0xa6c62ce5, 0x3a95, 0x41bb, \
    { 0xb0, 0xf1, 0x8c, 0xd4, 0xf4, 0xca, 0x00, 0xe3 }}

class NS_NO_VTABLE IUrlClassifierUITelemetry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IURLCLASSIFIERUITELEMETRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = IUrlClassifierUITelemetry;

  enum {
    WARNING_MALWARE_PAGE_TOP = 1U,
    WARNING_MALWARE_PAGE_TOP_WHY_BLOCKED = 2U,
    WARNING_MALWARE_PAGE_TOP_GET_ME_OUT_OF_HERE = 3U,
    WARNING_MALWARE_PAGE_TOP_IGNORE_WARNING = 4U,
    WARNING_MALWARE_PAGE_FRAME = 5U,
    WARNING_MALWARE_PAGE_FRAME_WHY_BLOCKED = 6U,
    WARNING_MALWARE_PAGE_FRAME_GET_ME_OUT_OF_HERE = 7U,
    WARNING_MALWARE_PAGE_FRAME_IGNORE_WARNING = 8U,
    WARNING_PHISHING_PAGE_TOP = 9U,
    WARNING_PHISHING_PAGE_TOP_WHY_BLOCKED = 10U,
    WARNING_PHISHING_PAGE_TOP_GET_ME_OUT_OF_HERE = 11U,
    WARNING_PHISHING_PAGE_TOP_IGNORE_WARNING = 12U,
    WARNING_PHISHING_PAGE_FRAME = 13U,
    WARNING_PHISHING_PAGE_FRAME_WHY_BLOCKED = 14U,
    WARNING_PHISHING_PAGE_FRAME_GET_ME_OUT_OF_HERE = 15U,
    WARNING_PHISHING_PAGE_FRAME_IGNORE_WARNING = 16U,
    WARNING_UNWANTED_PAGE_TOP = 17U,
    WARNING_UNWANTED_PAGE_TOP_WHY_BLOCKED = 18U,
    WARNING_UNWANTED_PAGE_TOP_GET_ME_OUT_OF_HERE = 19U,
    WARNING_UNWANTED_PAGE_TOP_IGNORE_WARNING = 20U,
    WARNING_UNWANTED_PAGE_FRAME = 21U,
    WARNING_UNWANTED_PAGE_FRAME_WHY_BLOCKED = 22U,
    WARNING_UNWANTED_PAGE_FRAME_GET_ME_OUT_OF_HERE = 23U,
    WARNING_UNWANTED_PAGE_FRAME_IGNORE_WARNING = 24U,
    WARNING_HARMFUL_PAGE_TOP = 25U,
    WARNING_HARMFUL_PAGE_TOP_WHY_BLOCKED = 26U,
    WARNING_HARMFUL_PAGE_TOP_GET_ME_OUT_OF_HERE = 27U,
    WARNING_HARMFUL_PAGE_TOP_IGNORE_WARNING = 28U,
    WARNING_HARMFUL_PAGE_FRAME = 29U,
    WARNING_HARMFUL_PAGE_FRAME_WHY_BLOCKED = 30U,
    WARNING_HARMFUL_PAGE_FRAME_GET_ME_OUT_OF_HERE = 31U,
    WARNING_HARMFUL_PAGE_FRAME_IGNORE_WARNING = 32U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(IUrlClassifierUITelemetry, IURLCLASSIFIERUITELEMETRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IURLCLASSIFIERUITELEMETRY \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IURLCLASSIFIERUITELEMETRY \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IURLCLASSIFIERUITELEMETRY(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IURLCLASSIFIERUITELEMETRY(_to) \


#endif /* __gen_IUrlClassifierUITelemetry_h__ */
