/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableInputStream.idl
 */

#ifndef __gen_nsIScriptableInputStream_h__
#define __gen_nsIScriptableInputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    nsIScriptableInputStream */
#define NS_ISCRIPTABLEINPUTSTREAM_IID_STR "3fce9015-472a-4080-ac3e-cd875dbe361e"

#define NS_ISCRIPTABLEINPUTSTREAM_IID \
  {0x3fce9015, 0x472a, 0x4080, \
    { 0xac, 0x3e, 0xcd, 0x87, 0x5d, 0xbe, 0x36, 0x1e }}

class NS_NO_VTABLE nsIScriptableInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableInputStream;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void init (in nsIInputStream aInputStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *aInputStream) = 0;

  /* unsigned long long available (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Available(uint64_t *_retval) = 0;

  /* string read (in unsigned long aCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Read(uint32_t aCount, char * *_retval) = 0;

  /* ACString readBytes (in unsigned long aCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadBytes(uint32_t aCount, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableInputStream, NS_ISCRIPTABLEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLEINPUTSTREAM \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD Init(nsIInputStream *aInputStream) override; \
  NS_IMETHOD Available(uint64_t *_retval) override; \
  NS_IMETHOD Read(uint32_t aCount, char * *_retval) override; \
  NS_IMETHOD ReadBytes(uint32_t aCount, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLEINPUTSTREAM \
  nsresult Close(void); \
  nsresult Init(nsIInputStream *aInputStream); \
  nsresult Available(uint64_t *_retval); \
  nsresult Read(uint32_t aCount, char * *_retval); \
  nsresult ReadBytes(uint32_t aCount, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLEINPUTSTREAM(_to) \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD Init(nsIInputStream *aInputStream) override { return _to Init(aInputStream); } \
  NS_IMETHOD Available(uint64_t *_retval) override { return _to Available(_retval); } \
  NS_IMETHOD Read(uint32_t aCount, char * *_retval) override { return _to Read(aCount, _retval); } \
  NS_IMETHOD ReadBytes(uint32_t aCount, nsACString& _retval) override { return _to ReadBytes(aCount, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLEINPUTSTREAM(_to) \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD Init(nsIInputStream *aInputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aInputStream); } \
  NS_IMETHOD Available(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Available(_retval); } \
  NS_IMETHOD Read(uint32_t aCount, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read(aCount, _retval); } \
  NS_IMETHOD ReadBytes(uint32_t aCount, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadBytes(aCount, _retval); } 


#endif /* __gen_nsIScriptableInputStream_h__ */
