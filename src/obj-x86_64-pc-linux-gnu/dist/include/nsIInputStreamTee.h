/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamTee.idl
 */

#ifndef __gen_nsIInputStreamTee_h__
#define __gen_nsIInputStreamTee_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStream; /* forward declaration */

class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIInputStreamTee */
#define NS_IINPUTSTREAMTEE_IID_STR "90a9d790-3bca-421e-a73b-98f68e13c917"

#define NS_IINPUTSTREAMTEE_IID \
  {0x90a9d790, 0x3bca, 0x421e, \
    { 0xa7, 0x3b, 0x98, 0xf6, 0x8e, 0x13, 0xc9, 0x17 }}

class NS_NO_VTABLE nsIInputStreamTee : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMTEE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamTee;

  /* attribute nsIInputStream source; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSource(nsIInputStream **aSource) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSource(nsIInputStream *aSource) = 0;

  /* attribute nsIOutputStream sink; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSink(nsIOutputStream **aSink) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSink(nsIOutputStream *aSink) = 0;

  /* attribute nsIEventTarget eventTarget; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEventTarget(nsIEventTarget **aEventTarget) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEventTarget(nsIEventTarget *aEventTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamTee, NS_IINPUTSTREAMTEE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMTEE \
  NS_IMETHOD GetSource(nsIInputStream **aSource) override; \
  NS_IMETHOD SetSource(nsIInputStream *aSource) override; \
  NS_IMETHOD GetSink(nsIOutputStream **aSink) override; \
  NS_IMETHOD SetSink(nsIOutputStream *aSink) override; \
  NS_IMETHOD GetEventTarget(nsIEventTarget **aEventTarget) override; \
  NS_IMETHOD SetEventTarget(nsIEventTarget *aEventTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMTEE \
  nsresult GetSource(nsIInputStream **aSource); \
  nsresult SetSource(nsIInputStream *aSource); \
  nsresult GetSink(nsIOutputStream **aSink); \
  nsresult SetSink(nsIOutputStream *aSink); \
  nsresult GetEventTarget(nsIEventTarget **aEventTarget); \
  nsresult SetEventTarget(nsIEventTarget *aEventTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMTEE(_to) \
  NS_IMETHOD GetSource(nsIInputStream **aSource) override { return _to GetSource(aSource); } \
  NS_IMETHOD SetSource(nsIInputStream *aSource) override { return _to SetSource(aSource); } \
  NS_IMETHOD GetSink(nsIOutputStream **aSink) override { return _to GetSink(aSink); } \
  NS_IMETHOD SetSink(nsIOutputStream *aSink) override { return _to SetSink(aSink); } \
  NS_IMETHOD GetEventTarget(nsIEventTarget **aEventTarget) override { return _to GetEventTarget(aEventTarget); } \
  NS_IMETHOD SetEventTarget(nsIEventTarget *aEventTarget) override { return _to SetEventTarget(aEventTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMTEE(_to) \
  NS_IMETHOD GetSource(nsIInputStream **aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSource(aSource); } \
  NS_IMETHOD SetSource(nsIInputStream *aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSource(aSource); } \
  NS_IMETHOD GetSink(nsIOutputStream **aSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSink(aSink); } \
  NS_IMETHOD SetSink(nsIOutputStream *aSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSink(aSink); } \
  NS_IMETHOD GetEventTarget(nsIEventTarget **aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEventTarget(aEventTarget); } \
  NS_IMETHOD SetEventTarget(nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEventTarget(aEventTarget); } 

// factory methods
extern nsresult
NS_NewInputStreamTee(nsIInputStream **tee, // read from this input stream
                     nsIInputStream *source,
                     nsIOutputStream *sink);
extern nsresult
NS_NewInputStreamTeeAsync(nsIInputStream **tee, // read from this input stream
                     nsIInputStream *source,
                     nsIOutputStream *sink,
                     nsIEventTarget *eventTarget);

#endif /* __gen_nsIInputStreamTee_h__ */
