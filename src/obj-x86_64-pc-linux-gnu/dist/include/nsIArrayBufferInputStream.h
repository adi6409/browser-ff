/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIArrayBufferInputStream.idl
 */

#ifndef __gen_nsIArrayBufferInputStream_h__
#define __gen_nsIArrayBufferInputStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIArrayBufferInputStream */
#define NS_IARRAYBUFFERINPUTSTREAM_IID_STR "3014dde6-aa1c-41db-87d0-48764a3710f6"

#define NS_IARRAYBUFFERINPUTSTREAM_IID \
  {0x3014dde6, 0xaa1c, 0x41db, \
    { 0x87, 0xd0, 0x48, 0x76, 0x4a, 0x37, 0x10, 0xf6 }}

class NS_NO_VTABLE nsIArrayBufferInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IARRAYBUFFERINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIArrayBufferInputStream;

  /* void setData (in jsval buffer, in uint64_t byteOffset, in uint64_t byteLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetData(JS::HandleValue buffer, uint64_t byteOffset, uint64_t byteLen) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIArrayBufferInputStream, NS_IARRAYBUFFERINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIARRAYBUFFERINPUTSTREAM \
  NS_IMETHOD SetData(JS::HandleValue buffer, uint64_t byteOffset, uint64_t byteLen) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIARRAYBUFFERINPUTSTREAM \
  nsresult SetData(JS::HandleValue buffer, uint64_t byteOffset, uint64_t byteLen); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIARRAYBUFFERINPUTSTREAM(_to) \
  NS_IMETHOD SetData(JS::HandleValue buffer, uint64_t byteOffset, uint64_t byteLen) override { return _to SetData(buffer, byteOffset, byteLen); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIARRAYBUFFERINPUTSTREAM(_to) \
  NS_IMETHOD SetData(JS::HandleValue buffer, uint64_t byteOffset, uint64_t byteLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetData(buffer, byteOffset, byteLen); } 


#endif /* __gen_nsIArrayBufferInputStream_h__ */
