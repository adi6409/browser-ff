/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertValidity.idl
 */

#ifndef __gen_nsIX509CertValidity_h__
#define __gen_nsIX509CertValidity_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIX509CertValidity */
#define NS_IX509CERTVALIDITY_IID_STR "e701dfd8-1dd1-11b2-a172-ffa6cc6156ad"

#define NS_IX509CERTVALIDITY_IID \
  {0xe701dfd8, 0x1dd1, 0x11b2, \
    { 0xa1, 0x72, 0xff, 0xa6, 0xcc, 0x61, 0x56, 0xad }}

class NS_NO_VTABLE nsIX509CertValidity : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IX509CERTVALIDITY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIX509CertValidity;

  /* readonly attribute PRTime notBefore; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotBefore(PRTime *aNotBefore) = 0;

  /* [must_use] readonly attribute AString notBeforeLocalTime; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNotBeforeLocalTime(nsAString& aNotBeforeLocalTime) = 0;

  /* readonly attribute AString notBeforeLocalDay; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotBeforeLocalDay(nsAString& aNotBeforeLocalDay) = 0;

  /* [must_use] readonly attribute AString notBeforeGMT; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNotBeforeGMT(nsAString& aNotBeforeGMT) = 0;

  /* readonly attribute PRTime notAfter; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotAfter(PRTime *aNotAfter) = 0;

  /* [must_use] readonly attribute AString notAfterLocalTime; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNotAfterLocalTime(nsAString& aNotAfterLocalTime) = 0;

  /* readonly attribute AString notAfterLocalDay; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotAfterLocalDay(nsAString& aNotAfterLocalDay) = 0;

  /* [must_use] readonly attribute AString notAfterGMT; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNotAfterGMT(nsAString& aNotAfterGMT) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIX509CertValidity, NS_IX509CERTVALIDITY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIX509CERTVALIDITY \
  NS_IMETHOD GetNotBefore(PRTime *aNotBefore) override; \
  [[nodiscard]] NS_IMETHOD GetNotBeforeLocalTime(nsAString& aNotBeforeLocalTime) override; \
  NS_IMETHOD GetNotBeforeLocalDay(nsAString& aNotBeforeLocalDay) override; \
  [[nodiscard]] NS_IMETHOD GetNotBeforeGMT(nsAString& aNotBeforeGMT) override; \
  NS_IMETHOD GetNotAfter(PRTime *aNotAfter) override; \
  [[nodiscard]] NS_IMETHOD GetNotAfterLocalTime(nsAString& aNotAfterLocalTime) override; \
  NS_IMETHOD GetNotAfterLocalDay(nsAString& aNotAfterLocalDay) override; \
  [[nodiscard]] NS_IMETHOD GetNotAfterGMT(nsAString& aNotAfterGMT) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIX509CERTVALIDITY \
  nsresult GetNotBefore(PRTime *aNotBefore); \
  [[nodiscard]] nsresult GetNotBeforeLocalTime(nsAString& aNotBeforeLocalTime); \
  nsresult GetNotBeforeLocalDay(nsAString& aNotBeforeLocalDay); \
  [[nodiscard]] nsresult GetNotBeforeGMT(nsAString& aNotBeforeGMT); \
  nsresult GetNotAfter(PRTime *aNotAfter); \
  [[nodiscard]] nsresult GetNotAfterLocalTime(nsAString& aNotAfterLocalTime); \
  nsresult GetNotAfterLocalDay(nsAString& aNotAfterLocalDay); \
  [[nodiscard]] nsresult GetNotAfterGMT(nsAString& aNotAfterGMT); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIX509CERTVALIDITY(_to) \
  NS_IMETHOD GetNotBefore(PRTime *aNotBefore) override { return _to GetNotBefore(aNotBefore); } \
  [[nodiscard]] NS_IMETHOD GetNotBeforeLocalTime(nsAString& aNotBeforeLocalTime) override { return _to GetNotBeforeLocalTime(aNotBeforeLocalTime); } \
  NS_IMETHOD GetNotBeforeLocalDay(nsAString& aNotBeforeLocalDay) override { return _to GetNotBeforeLocalDay(aNotBeforeLocalDay); } \
  [[nodiscard]] NS_IMETHOD GetNotBeforeGMT(nsAString& aNotBeforeGMT) override { return _to GetNotBeforeGMT(aNotBeforeGMT); } \
  NS_IMETHOD GetNotAfter(PRTime *aNotAfter) override { return _to GetNotAfter(aNotAfter); } \
  [[nodiscard]] NS_IMETHOD GetNotAfterLocalTime(nsAString& aNotAfterLocalTime) override { return _to GetNotAfterLocalTime(aNotAfterLocalTime); } \
  NS_IMETHOD GetNotAfterLocalDay(nsAString& aNotAfterLocalDay) override { return _to GetNotAfterLocalDay(aNotAfterLocalDay); } \
  [[nodiscard]] NS_IMETHOD GetNotAfterGMT(nsAString& aNotAfterGMT) override { return _to GetNotAfterGMT(aNotAfterGMT); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIX509CERTVALIDITY(_to) \
  NS_IMETHOD GetNotBefore(PRTime *aNotBefore) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotBefore(aNotBefore); } \
  [[nodiscard]] NS_IMETHOD GetNotBeforeLocalTime(nsAString& aNotBeforeLocalTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotBeforeLocalTime(aNotBeforeLocalTime); } \
  NS_IMETHOD GetNotBeforeLocalDay(nsAString& aNotBeforeLocalDay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotBeforeLocalDay(aNotBeforeLocalDay); } \
  [[nodiscard]] NS_IMETHOD GetNotBeforeGMT(nsAString& aNotBeforeGMT) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotBeforeGMT(aNotBeforeGMT); } \
  NS_IMETHOD GetNotAfter(PRTime *aNotAfter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotAfter(aNotAfter); } \
  [[nodiscard]] NS_IMETHOD GetNotAfterLocalTime(nsAString& aNotAfterLocalTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotAfterLocalTime(aNotAfterLocalTime); } \
  NS_IMETHOD GetNotAfterLocalDay(nsAString& aNotAfterLocalDay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotAfterLocalDay(aNotAfterLocalDay); } \
  [[nodiscard]] NS_IMETHOD GetNotAfterGMT(nsAString& aNotAfterGMT) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotAfterGMT(aNotAfterGMT); } 


#endif /* __gen_nsIX509CertValidity_h__ */
