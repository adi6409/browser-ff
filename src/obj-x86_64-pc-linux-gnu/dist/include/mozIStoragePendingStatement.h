/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStoragePendingStatement.idl
 */

#ifndef __gen_mozIStoragePendingStatement_h__
#define __gen_mozIStoragePendingStatement_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIStoragePendingStatement */
#define MOZISTORAGEPENDINGSTATEMENT_IID_STR "00da7d20-3768-4398-bedc-e310c324b3f0"

#define MOZISTORAGEPENDINGSTATEMENT_IID \
  {0x00da7d20, 0x3768, 0x4398, \
    { 0xbe, 0xdc, 0xe3, 0x10, 0xc3, 0x24, 0xb3, 0xf0 }}

class NS_NO_VTABLE mozIStoragePendingStatement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEPENDINGSTATEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStoragePendingStatement;

  /* void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStoragePendingStatement, MOZISTORAGEPENDINGSTATEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEPENDINGSTATEMENT \
  NS_IMETHOD Cancel(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEPENDINGSTATEMENT \
  nsresult Cancel(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEPENDINGSTATEMENT(_to) \
  NS_IMETHOD Cancel(void) override { return _to Cancel(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEPENDINGSTATEMENT(_to) \
  NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } 


#endif /* __gen_mozIStoragePendingStatement_h__ */
