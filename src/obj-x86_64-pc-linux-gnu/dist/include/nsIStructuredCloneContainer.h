/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIStructuredCloneContainer.idl
 */

#ifndef __gen_nsIStructuredCloneContainer_h__
#define __gen_nsIStructuredCloneContainer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIVariant; /* forward declaration */

#include "js/TypeDecls.h"

/* starting interface:    nsIStructuredCloneContainer */
#define NS_ISTRUCTUREDCLONECONTAINER_IID_STR "c664aae7-0d67-4155-a2dd-a3861778626f"

#define NS_ISTRUCTUREDCLONECONTAINER_IID \
  {0xc664aae7, 0x0d67, 0x4155, \
    { 0xa2, 0xdd, 0xa3, 0x86, 0x17, 0x78, 0x62, 0x6f }}

class NS_NO_VTABLE nsIStructuredCloneContainer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTRUCTUREDCLONECONTAINER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStructuredCloneContainer;

  /* [implicit_jscontext,noscript] void initFromJSVal (in jsval aData); */
  NS_IMETHOD InitFromJSVal(JS::HandleValue aData, JSContext* cx) = 0;

  /* void initFromBase64 (in AString aData, in unsigned long aFormatVersion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitFromBase64(const nsAString& aData, uint32_t aFormatVersion) = 0;

  /* [implicit_jscontext] jsval deserializeToJsval (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeserializeToJsval(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] nsIVariant deserializeToVariant (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeserializeToVariant(JSContext* cx, nsIVariant **_retval) = 0;

  /* AString getDataAsBase64 (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDataAsBase64(nsAString& _retval) = 0;

  /* readonly attribute unsigned long long serializedNBytes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSerializedNBytes(uint64_t *aSerializedNBytes) = 0;

  /* readonly attribute unsigned long formatVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFormatVersion(uint32_t *aFormatVersion) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStructuredCloneContainer, NS_ISTRUCTUREDCLONECONTAINER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTRUCTUREDCLONECONTAINER \
  NS_IMETHOD InitFromJSVal(JS::HandleValue aData, JSContext* cx) override; \
  NS_IMETHOD InitFromBase64(const nsAString& aData, uint32_t aFormatVersion) override; \
  NS_IMETHOD DeserializeToJsval(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD DeserializeToVariant(JSContext* cx, nsIVariant **_retval) override; \
  NS_IMETHOD GetDataAsBase64(nsAString& _retval) override; \
  NS_IMETHOD GetSerializedNBytes(uint64_t *aSerializedNBytes) override; \
  NS_IMETHOD GetFormatVersion(uint32_t *aFormatVersion) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTRUCTUREDCLONECONTAINER \
  nsresult InitFromJSVal(JS::HandleValue aData, JSContext* cx); \
  nsresult InitFromBase64(const nsAString& aData, uint32_t aFormatVersion); \
  nsresult DeserializeToJsval(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult DeserializeToVariant(JSContext* cx, nsIVariant **_retval); \
  nsresult GetDataAsBase64(nsAString& _retval); \
  nsresult GetSerializedNBytes(uint64_t *aSerializedNBytes); \
  nsresult GetFormatVersion(uint32_t *aFormatVersion); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTRUCTUREDCLONECONTAINER(_to) \
  NS_IMETHOD InitFromJSVal(JS::HandleValue aData, JSContext* cx) override { return _to InitFromJSVal(aData, cx); } \
  NS_IMETHOD InitFromBase64(const nsAString& aData, uint32_t aFormatVersion) override { return _to InitFromBase64(aData, aFormatVersion); } \
  NS_IMETHOD DeserializeToJsval(JSContext* cx, JS::MutableHandleValue _retval) override { return _to DeserializeToJsval(cx, _retval); } \
  NS_IMETHOD DeserializeToVariant(JSContext* cx, nsIVariant **_retval) override { return _to DeserializeToVariant(cx, _retval); } \
  NS_IMETHOD GetDataAsBase64(nsAString& _retval) override { return _to GetDataAsBase64(_retval); } \
  NS_IMETHOD GetSerializedNBytes(uint64_t *aSerializedNBytes) override { return _to GetSerializedNBytes(aSerializedNBytes); } \
  NS_IMETHOD GetFormatVersion(uint32_t *aFormatVersion) override { return _to GetFormatVersion(aFormatVersion); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTRUCTUREDCLONECONTAINER(_to) \
  NS_IMETHOD InitFromJSVal(JS::HandleValue aData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitFromJSVal(aData, cx); } \
  NS_IMETHOD InitFromBase64(const nsAString& aData, uint32_t aFormatVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitFromBase64(aData, aFormatVersion); } \
  NS_IMETHOD DeserializeToJsval(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeserializeToJsval(cx, _retval); } \
  NS_IMETHOD DeserializeToVariant(JSContext* cx, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeserializeToVariant(cx, _retval); } \
  NS_IMETHOD GetDataAsBase64(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDataAsBase64(_retval); } \
  NS_IMETHOD GetSerializedNBytes(uint64_t *aSerializedNBytes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSerializedNBytes(aSerializedNBytes); } \
  NS_IMETHOD GetFormatVersion(uint32_t *aFormatVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFormatVersion(aFormatVersion); } 


#endif /* __gen_nsIStructuredCloneContainer_h__ */
