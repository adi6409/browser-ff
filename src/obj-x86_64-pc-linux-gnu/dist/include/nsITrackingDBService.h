/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsITrackingDBService.idl
 */

#ifndef __gen_nsITrackingDBService_h__
#define __gen_nsITrackingDBService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIAsyncInputStream; /* forward declaration */


/* starting interface:    nsITrackingDBService */
#define NS_ITRACKINGDBSERVICE_IID_STR "650934db-1939-4424-be26-6ffb0375424d"

#define NS_ITRACKINGDBSERVICE_IID \
  {0x650934db, 0x1939, 0x4424, \
    { 0xbe, 0x26, 0x6f, 0xfb, 0x03, 0x75, 0x42, 0x4d }}

class NS_NO_VTABLE nsITrackingDBService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRACKINGDBSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITrackingDBService;

  /* void recordContentBlockingLog (in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordContentBlockingLog(const nsACString& data) = 0;

  /* Promise saveEvents (in AString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveEvents(const nsAString& data, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise clearAll (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearAll(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise clearSince (in int64_t since); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSince(int64_t since, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getEventsByDateRange (in int64_t dateFrom, in int64_t dateTo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEventsByDateRange(int64_t dateFrom, int64_t dateTo, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise sumAllEvents (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SumAllEvents(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getEarliestRecordedDate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEarliestRecordedDate(::mozilla::dom::Promise * * _retval) = 0;

  enum {
    OTHER_COOKIES_BLOCKED_ID = 0U,
    TRACKERS_ID = 1U,
    TRACKING_COOKIES_ID = 2U,
    CRYPTOMINERS_ID = 3U,
    FINGERPRINTERS_ID = 4U,
    SOCIAL_ID = 5U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITrackingDBService, NS_ITRACKINGDBSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRACKINGDBSERVICE \
  NS_IMETHOD RecordContentBlockingLog(const nsACString& data) override; \
  NS_IMETHOD SaveEvents(const nsAString& data, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ClearAll(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ClearSince(int64_t since, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetEventsByDateRange(int64_t dateFrom, int64_t dateTo, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SumAllEvents(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetEarliestRecordedDate(::mozilla::dom::Promise * * _retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRACKINGDBSERVICE \
  nsresult RecordContentBlockingLog(const nsACString& data); \
  nsresult SaveEvents(const nsAString& data, ::mozilla::dom::Promise * * _retval); \
  nsresult ClearAll(::mozilla::dom::Promise * * _retval); \
  nsresult ClearSince(int64_t since, ::mozilla::dom::Promise * * _retval); \
  nsresult GetEventsByDateRange(int64_t dateFrom, int64_t dateTo, ::mozilla::dom::Promise * * _retval); \
  nsresult SumAllEvents(::mozilla::dom::Promise * * _retval); \
  nsresult GetEarliestRecordedDate(::mozilla::dom::Promise * * _retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRACKINGDBSERVICE(_to) \
  NS_IMETHOD RecordContentBlockingLog(const nsACString& data) override { return _to RecordContentBlockingLog(data); } \
  NS_IMETHOD SaveEvents(const nsAString& data, ::mozilla::dom::Promise * * _retval) override { return _to SaveEvents(data, _retval); } \
  NS_IMETHOD ClearAll(::mozilla::dom::Promise * * _retval) override { return _to ClearAll(_retval); } \
  NS_IMETHOD ClearSince(int64_t since, ::mozilla::dom::Promise * * _retval) override { return _to ClearSince(since, _retval); } \
  NS_IMETHOD GetEventsByDateRange(int64_t dateFrom, int64_t dateTo, ::mozilla::dom::Promise * * _retval) override { return _to GetEventsByDateRange(dateFrom, dateTo, _retval); } \
  NS_IMETHOD SumAllEvents(::mozilla::dom::Promise * * _retval) override { return _to SumAllEvents(_retval); } \
  NS_IMETHOD GetEarliestRecordedDate(::mozilla::dom::Promise * * _retval) override { return _to GetEarliestRecordedDate(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRACKINGDBSERVICE(_to) \
  NS_IMETHOD RecordContentBlockingLog(const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordContentBlockingLog(data); } \
  NS_IMETHOD SaveEvents(const nsAString& data, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveEvents(data, _retval); } \
  NS_IMETHOD ClearAll(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearAll(_retval); } \
  NS_IMETHOD ClearSince(int64_t since, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSince(since, _retval); } \
  NS_IMETHOD GetEventsByDateRange(int64_t dateFrom, int64_t dateTo, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEventsByDateRange(dateFrom, dateTo, _retval); } \
  NS_IMETHOD SumAllEvents(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SumAllEvents(_retval); } \
  NS_IMETHOD GetEarliestRecordedDate(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEarliestRecordedDate(_retval); } \


#endif /* __gen_nsITrackingDBService_h__ */
