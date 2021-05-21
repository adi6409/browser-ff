/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/xulstore/nsIXULStore.idl
 */

#ifndef __gen_nsIXULStore_h__
#define __gen_nsIXULStore_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIStringEnumerator; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIXULStore */
#define NS_IXULSTORE_IID_STR "987c4b35-c426-4dd7-ad49-3c9fa4c65d20"

#define NS_IXULSTORE_IID \
  {0x987c4b35, 0xc426, 0x4dd7, \
    { 0xad, 0x49, 0x3c, 0x9f, 0xa4, 0xc6, 0x5d, 0x20 }}

class NS_NO_VTABLE nsIXULStore : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXULSTORE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXULStore;

  /* void persist (in Node aNode, in AString attr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Persist(nsINode *aNode, const nsAString& attr) = 0;

  /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, const nsAString& value) = 0;

  /* bool hasValue (in AString doc, in AString id, in AString attr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasValue(const nsAString& doc, const nsAString& id, const nsAString& attr, bool *_retval) = 0;

  /* AString getValue (in AString doc, in AString id, in AString attr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, nsAString& _retval) = 0;

  /* void removeValue (in AString doc, in AString id, in AString attr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveValue(const nsAString& doc, const nsAString& id, const nsAString& attr) = 0;

  /* void removeDocument (in AString doc); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDocument(const nsAString& doc) = 0;

  /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIDsEnumerator(const nsAString& doc, nsIStringEnumerator **_retval) = 0;

  /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAttributeEnumerator(const nsAString& doc, const nsAString& id, nsIStringEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXULStore, NS_IXULSTORE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXULSTORE \
  NS_IMETHOD Persist(nsINode *aNode, const nsAString& attr) override; \
  NS_IMETHOD SetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, const nsAString& value) override; \
  NS_IMETHOD HasValue(const nsAString& doc, const nsAString& id, const nsAString& attr, bool *_retval) override; \
  NS_IMETHOD GetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, nsAString& _retval) override; \
  NS_IMETHOD RemoveValue(const nsAString& doc, const nsAString& id, const nsAString& attr) override; \
  NS_IMETHOD RemoveDocument(const nsAString& doc) override; \
  NS_IMETHOD GetIDsEnumerator(const nsAString& doc, nsIStringEnumerator **_retval) override; \
  NS_IMETHOD GetAttributeEnumerator(const nsAString& doc, const nsAString& id, nsIStringEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXULSTORE \
  nsresult Persist(nsINode *aNode, const nsAString& attr); \
  nsresult SetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, const nsAString& value); \
  nsresult HasValue(const nsAString& doc, const nsAString& id, const nsAString& attr, bool *_retval); \
  nsresult GetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, nsAString& _retval); \
  nsresult RemoveValue(const nsAString& doc, const nsAString& id, const nsAString& attr); \
  nsresult RemoveDocument(const nsAString& doc); \
  nsresult GetIDsEnumerator(const nsAString& doc, nsIStringEnumerator **_retval); \
  nsresult GetAttributeEnumerator(const nsAString& doc, const nsAString& id, nsIStringEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXULSTORE(_to) \
  NS_IMETHOD Persist(nsINode *aNode, const nsAString& attr) override { return _to Persist(aNode, attr); } \
  NS_IMETHOD SetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, const nsAString& value) override { return _to SetValue(doc, id, attr, value); } \
  NS_IMETHOD HasValue(const nsAString& doc, const nsAString& id, const nsAString& attr, bool *_retval) override { return _to HasValue(doc, id, attr, _retval); } \
  NS_IMETHOD GetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, nsAString& _retval) override { return _to GetValue(doc, id, attr, _retval); } \
  NS_IMETHOD RemoveValue(const nsAString& doc, const nsAString& id, const nsAString& attr) override { return _to RemoveValue(doc, id, attr); } \
  NS_IMETHOD RemoveDocument(const nsAString& doc) override { return _to RemoveDocument(doc); } \
  NS_IMETHOD GetIDsEnumerator(const nsAString& doc, nsIStringEnumerator **_retval) override { return _to GetIDsEnumerator(doc, _retval); } \
  NS_IMETHOD GetAttributeEnumerator(const nsAString& doc, const nsAString& id, nsIStringEnumerator **_retval) override { return _to GetAttributeEnumerator(doc, id, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXULSTORE(_to) \
  NS_IMETHOD Persist(nsINode *aNode, const nsAString& attr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Persist(aNode, attr); } \
  NS_IMETHOD SetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(doc, id, attr, value); } \
  NS_IMETHOD HasValue(const nsAString& doc, const nsAString& id, const nsAString& attr, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasValue(doc, id, attr, _retval); } \
  NS_IMETHOD GetValue(const nsAString& doc, const nsAString& id, const nsAString& attr, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(doc, id, attr, _retval); } \
  NS_IMETHOD RemoveValue(const nsAString& doc, const nsAString& id, const nsAString& attr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveValue(doc, id, attr); } \
  NS_IMETHOD RemoveDocument(const nsAString& doc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDocument(doc); } \
  NS_IMETHOD GetIDsEnumerator(const nsAString& doc, nsIStringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIDsEnumerator(doc, _retval); } \
  NS_IMETHOD GetAttributeEnumerator(const nsAString& doc, const nsAString& id, nsIStringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAttributeEnumerator(doc, id, _retval); } 


#endif /* __gen_nsIXULStore_h__ */
