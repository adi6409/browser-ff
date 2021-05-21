/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageError.idl
 */

#ifndef __gen_mozIStorageError_h__
#define __gen_mozIStorageError_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#ifdef ERROR
#undef ERROR
#endif

/* starting interface:    mozIStorageError */
#define MOZISTORAGEERROR_IID_STR "1f350f96-7023-434a-8864-40a1c493aac1"

#define MOZISTORAGEERROR_IID \
  {0x1f350f96, 0x7023, 0x434a, \
    { 0x88, 0x64, 0x40, 0xa1, 0xc4, 0x93, 0xaa, 0xc1 }}

class NS_NO_VTABLE mozIStorageError : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEERROR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageError;

  enum {
    ERROR = 1,
    INTERNAL = 2,
    PERM = 3,
    ABORT = 4,
    BUSY = 5,
    LOCKED = 6,
    NOMEM = 7,
    READONLY = 8,
    INTERRUPT = 9,
    IOERR = 10,
    CORRUPT = 11,
    FULL = 13,
    CANTOPEN = 14,
    EMPTY = 16,
    SCHEMA = 17,
    TOOBIG = 18,
    CONSTRAINT = 19,
    MISMATCH = 20,
    MISUSE = 21,
    NOLFS = 22,
    AUTH = 23,
    FORMAT = 24,
    RANGE = 25,
    NOTADB = 26
  };

  /* readonly attribute long result; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResult(int32_t *aResult) = 0;

  /* readonly attribute AUTF8String message; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMessage(nsACString& aMessage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageError, MOZISTORAGEERROR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEERROR \
  NS_IMETHOD GetResult(int32_t *aResult) override; \
  NS_IMETHOD GetMessage(nsACString& aMessage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEERROR \
  nsresult GetResult(int32_t *aResult); \
  nsresult GetMessage(nsACString& aMessage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEERROR(_to) \
  NS_IMETHOD GetResult(int32_t *aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD GetMessage(nsACString& aMessage) override { return _to GetMessage(aMessage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEERROR(_to) \
  NS_IMETHOD GetResult(int32_t *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD GetMessage(nsACString& aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMessage(aMessage); } 


#endif /* __gen_mozIStorageError_h__ */
