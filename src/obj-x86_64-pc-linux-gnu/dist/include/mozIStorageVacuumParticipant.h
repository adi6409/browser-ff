/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageVacuumParticipant.idl
 */

#ifndef __gen_mozIStorageVacuumParticipant_h__
#define __gen_mozIStorageVacuumParticipant_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageConnection; /* forward declaration */


/* starting interface:    mozIStorageVacuumParticipant */
#define MOZISTORAGEVACUUMPARTICIPANT_IID_STR "8f367508-1d9a-4d3f-be0c-ac11b6dd7dbf"

#define MOZISTORAGEVACUUMPARTICIPANT_IID \
  {0x8f367508, 0x1d9a, 0x4d3f, \
    { 0xbe, 0x0c, 0xac, 0x11, 0xb6, 0xdd, 0x7d, 0xbf }}

class NS_NO_VTABLE mozIStorageVacuumParticipant : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEVACUUMPARTICIPANT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageVacuumParticipant;

  /* readonly attribute long expectedDatabasePageSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpectedDatabasePageSize(int32_t *aExpectedDatabasePageSize) = 0;

  /* readonly attribute mozIStorageConnection databaseConnection; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDatabaseConnection(mozIStorageConnection **aDatabaseConnection) = 0;

  /* boolean onBeginVacuum (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnBeginVacuum(bool *_retval) = 0;

  /* void onEndVacuum (in boolean aSucceeded); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnEndVacuum(bool aSucceeded) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageVacuumParticipant, MOZISTORAGEVACUUMPARTICIPANT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEVACUUMPARTICIPANT \
  NS_IMETHOD GetExpectedDatabasePageSize(int32_t *aExpectedDatabasePageSize) override; \
  NS_IMETHOD GetDatabaseConnection(mozIStorageConnection **aDatabaseConnection) override; \
  NS_IMETHOD OnBeginVacuum(bool *_retval) override; \
  NS_IMETHOD OnEndVacuum(bool aSucceeded) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEVACUUMPARTICIPANT \
  nsresult GetExpectedDatabasePageSize(int32_t *aExpectedDatabasePageSize); \
  nsresult GetDatabaseConnection(mozIStorageConnection **aDatabaseConnection); \
  nsresult OnBeginVacuum(bool *_retval); \
  nsresult OnEndVacuum(bool aSucceeded); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEVACUUMPARTICIPANT(_to) \
  NS_IMETHOD GetExpectedDatabasePageSize(int32_t *aExpectedDatabasePageSize) override { return _to GetExpectedDatabasePageSize(aExpectedDatabasePageSize); } \
  NS_IMETHOD GetDatabaseConnection(mozIStorageConnection **aDatabaseConnection) override { return _to GetDatabaseConnection(aDatabaseConnection); } \
  NS_IMETHOD OnBeginVacuum(bool *_retval) override { return _to OnBeginVacuum(_retval); } \
  NS_IMETHOD OnEndVacuum(bool aSucceeded) override { return _to OnEndVacuum(aSucceeded); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEVACUUMPARTICIPANT(_to) \
  NS_IMETHOD GetExpectedDatabasePageSize(int32_t *aExpectedDatabasePageSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpectedDatabasePageSize(aExpectedDatabasePageSize); } \
  NS_IMETHOD GetDatabaseConnection(mozIStorageConnection **aDatabaseConnection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDatabaseConnection(aDatabaseConnection); } \
  NS_IMETHOD OnBeginVacuum(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBeginVacuum(_retval); } \
  NS_IMETHOD OnEndVacuum(bool aSucceeded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnEndVacuum(aSucceeded); } 


#endif /* __gen_mozIStorageVacuumParticipant_h__ */
