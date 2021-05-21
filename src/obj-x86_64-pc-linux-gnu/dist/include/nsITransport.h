/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITransport.idl
 */

#ifndef __gen_nsITransport_h__
#define __gen_nsITransport_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsITransportEventSink; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsITransport */
#define NS_ITRANSPORT_IID_STR "2a8c6334-a5e6-4ec3-9865-1256541446fb"

#define NS_ITRANSPORT_IID \
  {0x2a8c6334, 0xa5e6, 0x4ec3, \
    { 0x98, 0x65, 0x12, 0x56, 0x54, 0x14, 0x46, 0xfb }}

class NS_NO_VTABLE nsITransport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransport;

  enum {
    OPEN_BLOCKING = 1U,
    OPEN_UNBUFFERED = 2U
  };

  /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenInputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIInputStream **_retval) = 0;

  /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenOutputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIOutputStream **_retval) = 0;

  /* void close (in nsresult aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(nsresult aReason) = 0;

  /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEventSink(nsITransportEventSink *aSink, nsIEventTarget *aEventTarget) = 0;

  enum {
    STATUS_READING = 2152398856U,
    STATUS_WRITING = 2152398857U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransport, NS_ITRANSPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSPORT \
  NS_IMETHOD OpenInputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIInputStream **_retval) override; \
  NS_IMETHOD OpenOutputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIOutputStream **_retval) override; \
  NS_IMETHOD Close(nsresult aReason) override; \
  NS_IMETHOD SetEventSink(nsITransportEventSink *aSink, nsIEventTarget *aEventTarget) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSPORT \
  nsresult OpenInputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIInputStream **_retval); \
  nsresult OpenOutputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIOutputStream **_retval); \
  nsresult Close(nsresult aReason); \
  nsresult SetEventSink(nsITransportEventSink *aSink, nsIEventTarget *aEventTarget); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSPORT(_to) \
  NS_IMETHOD OpenInputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIInputStream **_retval) override { return _to OpenInputStream(aFlags, aSegmentSize, aSegmentCount, _retval); } \
  NS_IMETHOD OpenOutputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIOutputStream **_retval) override { return _to OpenOutputStream(aFlags, aSegmentSize, aSegmentCount, _retval); } \
  NS_IMETHOD Close(nsresult aReason) override { return _to Close(aReason); } \
  NS_IMETHOD SetEventSink(nsITransportEventSink *aSink, nsIEventTarget *aEventTarget) override { return _to SetEventSink(aSink, aEventTarget); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSPORT(_to) \
  NS_IMETHOD OpenInputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenInputStream(aFlags, aSegmentSize, aSegmentCount, _retval); } \
  NS_IMETHOD OpenOutputStream(uint32_t aFlags, uint32_t aSegmentSize, uint32_t aSegmentCount, nsIOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenOutputStream(aFlags, aSegmentSize, aSegmentCount, _retval); } \
  NS_IMETHOD Close(nsresult aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(aReason); } \
  NS_IMETHOD SetEventSink(nsITransportEventSink *aSink, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEventSink(aSink, aEventTarget); } \


/* starting interface:    nsITransportEventSink */
#define NS_ITRANSPORTEVENTSINK_IID_STR "eda4f520-67f7-484b-a691-8c3226a5b0a6"

#define NS_ITRANSPORTEVENTSINK_IID \
  {0xeda4f520, 0x67f7, 0x484b, \
    { 0xa6, 0x91, 0x8c, 0x32, 0x26, 0xa5, 0xb0, 0xa6 }}

class NS_NO_VTABLE nsITransportEventSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSPORTEVENTSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransportEventSink;

  /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTransportStatus(nsITransport *aTransport, nsresult aStatus, int64_t aProgress, int64_t aProgressMax) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransportEventSink, NS_ITRANSPORTEVENTSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSPORTEVENTSINK \
  NS_IMETHOD OnTransportStatus(nsITransport *aTransport, nsresult aStatus, int64_t aProgress, int64_t aProgressMax) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSPORTEVENTSINK \
  nsresult OnTransportStatus(nsITransport *aTransport, nsresult aStatus, int64_t aProgress, int64_t aProgressMax); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSPORTEVENTSINK(_to) \
  NS_IMETHOD OnTransportStatus(nsITransport *aTransport, nsresult aStatus, int64_t aProgress, int64_t aProgressMax) override { return _to OnTransportStatus(aTransport, aStatus, aProgress, aProgressMax); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSPORTEVENTSINK(_to) \
  NS_IMETHOD OnTransportStatus(nsITransport *aTransport, nsresult aStatus, int64_t aProgress, int64_t aProgressMax) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTransportStatus(aTransport, aStatus, aProgress, aProgressMax); } 


#endif /* __gen_nsITransport_h__ */
