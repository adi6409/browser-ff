/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/console/nsIConsoleAPIStorage.idl
 */

#ifndef __gen_nsIConsoleAPIStorage_h__
#define __gen_nsIConsoleAPIStorage_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIConsoleAPIStorage */
#define NS_ICONSOLEAPISTORAGE_IID_STR "9e32a7b6-c4d1-4d9a-87b9-1ef6b75c27a9"

#define NS_ICONSOLEAPISTORAGE_IID \
  {0x9e32a7b6, 0xc4d1, 0x4d9a, \
    { 0x87, 0xb9, 0x1e, 0xf6, 0xb7, 0x5c, 0x27, 0xa9 }}

class NS_NO_VTABLE nsIConsoleAPIStorage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONSOLEAPISTORAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIConsoleAPIStorage;

  /* jsval getEvents ([optional] in AString aId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEvents(const nsAString& aId, JS::MutableHandleValue _retval) = 0;

  /* void recordEvent (in AString aId, in AString aOuterId, in jsval aEvent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordEvent(const nsAString& aId, const nsAString& aOuterId, JS::HandleValue aEvent) = 0;

  /* void clearEvents ([optional] in AString aId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearEvents(const nsAString& aId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIConsoleAPIStorage, NS_ICONSOLEAPISTORAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONSOLEAPISTORAGE \
  NS_IMETHOD GetEvents(const nsAString& aId, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD RecordEvent(const nsAString& aId, const nsAString& aOuterId, JS::HandleValue aEvent) override; \
  NS_IMETHOD ClearEvents(const nsAString& aId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONSOLEAPISTORAGE \
  nsresult GetEvents(const nsAString& aId, JS::MutableHandleValue _retval); \
  nsresult RecordEvent(const nsAString& aId, const nsAString& aOuterId, JS::HandleValue aEvent); \
  nsresult ClearEvents(const nsAString& aId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONSOLEAPISTORAGE(_to) \
  NS_IMETHOD GetEvents(const nsAString& aId, JS::MutableHandleValue _retval) override { return _to GetEvents(aId, _retval); } \
  NS_IMETHOD RecordEvent(const nsAString& aId, const nsAString& aOuterId, JS::HandleValue aEvent) override { return _to RecordEvent(aId, aOuterId, aEvent); } \
  NS_IMETHOD ClearEvents(const nsAString& aId) override { return _to ClearEvents(aId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONSOLEAPISTORAGE(_to) \
  NS_IMETHOD GetEvents(const nsAString& aId, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEvents(aId, _retval); } \
  NS_IMETHOD RecordEvent(const nsAString& aId, const nsAString& aOuterId, JS::HandleValue aEvent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordEvent(aId, aOuterId, aEvent); } \
  NS_IMETHOD ClearEvents(const nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearEvents(aId); } 


#endif /* __gen_nsIConsoleAPIStorage_h__ */
