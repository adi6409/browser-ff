/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamLength.idl
 */

#ifndef __gen_nsIInputStreamLength_h__
#define __gen_nsIInputStreamLength_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEventTarget; /* forward declaration */

class nsIInputStreamLengthCallback; /* forward declaration */


/* starting interface:    nsIInputStreamLength */
#define NS_IINPUTSTREAMLENGTH_IID_STR "452d059f-9a9c-4434-8839-e10d1405647c"

#define NS_IINPUTSTREAMLENGTH_IID \
  {0x452d059f, 0x9a9c, 0x4434, \
    { 0x88, 0x39, 0xe1, 0x0d, 0x14, 0x05, 0x64, 0x7c }}

class NS_NO_VTABLE nsIInputStreamLength : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMLENGTH_IID)

  /* long long length (); */
  NS_IMETHOD Length(int64_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamLength, NS_IINPUTSTREAMLENGTH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMLENGTH \
  NS_IMETHOD Length(int64_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMLENGTH \
  nsresult Length(int64_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMLENGTH(_to) \
  NS_IMETHOD Length(int64_t *_retval) override { return _to Length(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMLENGTH(_to) \
  NS_IMETHOD Length(int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Length(_retval); } 


/* starting interface:    nsIAsyncInputStreamLength */
#define NS_IASYNCINPUTSTREAMLENGTH_IID_STR "b63f9ecf-4668-44a3-93bd-72dbc65a6125"

#define NS_IASYNCINPUTSTREAMLENGTH_IID \
  {0xb63f9ecf, 0x4668, 0x44a3, \
    { 0x93, 0xbd, 0x72, 0xdb, 0xc6, 0x5a, 0x61, 0x25 }}

class NS_NO_VTABLE nsIAsyncInputStreamLength : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCINPUTSTREAMLENGTH_IID)

  /* void asyncLengthWait (in nsIInputStreamLengthCallback aCallback, in nsIEventTarget aEventTarget); */
  NS_IMETHOD AsyncLengthWait(nsIInputStreamLengthCallback *aCallback, nsIEventTarget *aEventTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncInputStreamLength, NS_IASYNCINPUTSTREAMLENGTH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCINPUTSTREAMLENGTH \
  NS_IMETHOD AsyncLengthWait(nsIInputStreamLengthCallback *aCallback, nsIEventTarget *aEventTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCINPUTSTREAMLENGTH \
  nsresult AsyncLengthWait(nsIInputStreamLengthCallback *aCallback, nsIEventTarget *aEventTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCINPUTSTREAMLENGTH(_to) \
  NS_IMETHOD AsyncLengthWait(nsIInputStreamLengthCallback *aCallback, nsIEventTarget *aEventTarget) override { return _to AsyncLengthWait(aCallback, aEventTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCINPUTSTREAMLENGTH(_to) \
  NS_IMETHOD AsyncLengthWait(nsIInputStreamLengthCallback *aCallback, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncLengthWait(aCallback, aEventTarget); } 


/* starting interface:    nsIInputStreamLengthCallback */
#define NS_IINPUTSTREAMLENGTHCALLBACK_IID_STR "9c0c13b9-1b33-445d-8adb-a8a7866a6c06"

#define NS_IINPUTSTREAMLENGTHCALLBACK_IID \
  {0x9c0c13b9, 0x1b33, 0x445d, \
    { 0x8a, 0xdb, 0xa8, 0xa7, 0x86, 0x6a, 0x6c, 0x06 }}

class NS_NO_VTABLE nsIInputStreamLengthCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMLENGTHCALLBACK_IID)

  /* void onInputStreamLengthReady (in nsIAsyncInputStreamLength aStream, in long long aLength); */
  NS_IMETHOD OnInputStreamLengthReady(nsIAsyncInputStreamLength *aStream, int64_t aLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamLengthCallback, NS_IINPUTSTREAMLENGTHCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMLENGTHCALLBACK \
  NS_IMETHOD OnInputStreamLengthReady(nsIAsyncInputStreamLength *aStream, int64_t aLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMLENGTHCALLBACK \
  nsresult OnInputStreamLengthReady(nsIAsyncInputStreamLength *aStream, int64_t aLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMLENGTHCALLBACK(_to) \
  NS_IMETHOD OnInputStreamLengthReady(nsIAsyncInputStreamLength *aStream, int64_t aLength) override { return _to OnInputStreamLengthReady(aStream, aLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMLENGTHCALLBACK(_to) \
  NS_IMETHOD OnInputStreamLengthReady(nsIAsyncInputStreamLength *aStream, int64_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnInputStreamLengthReady(aStream, aLength); } 


#endif /* __gen_nsIInputStreamLength_h__ */
