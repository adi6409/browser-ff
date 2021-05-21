/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageResultSet.idl
 */

#ifndef __gen_mozIStorageResultSet_h__
#define __gen_mozIStorageResultSet_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageRow; /* forward declaration */


/* starting interface:    mozIStorageResultSet */
#define MOZISTORAGERESULTSET_IID_STR "18dd7953-076d-4598-8105-3e32ad26ab24"

#define MOZISTORAGERESULTSET_IID \
  {0x18dd7953, 0x076d, 0x4598, \
    { 0x81, 0x05, 0x3e, 0x32, 0xad, 0x26, 0xab, 0x24 }}

class NS_NO_VTABLE mozIStorageResultSet : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGERESULTSET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageResultSet;

  /* mozIStorageRow getNextRow (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextRow(mozIStorageRow **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageResultSet, MOZISTORAGERESULTSET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGERESULTSET \
  NS_IMETHOD GetNextRow(mozIStorageRow **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGERESULTSET \
  nsresult GetNextRow(mozIStorageRow **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGERESULTSET(_to) \
  NS_IMETHOD GetNextRow(mozIStorageRow **_retval) override { return _to GetNextRow(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGERESULTSET(_to) \
  NS_IMETHOD GetNextRow(mozIStorageRow **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextRow(_retval); } 


#endif /* __gen_mozIStorageResultSet_h__ */
