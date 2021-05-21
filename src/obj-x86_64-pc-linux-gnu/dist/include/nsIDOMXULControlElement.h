/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULControlElement.idl
 */

#ifndef __gen_nsIDOMXULControlElement_h__
#define __gen_nsIDOMXULControlElement_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIControllers; /* forward declaration */


/* starting interface:    nsIDOMXULControlElement */
#define NS_IDOMXULCONTROLELEMENT_IID_STR "bdc1d047-6d22-4813-bc50-638ccb349c7d"

#define NS_IDOMXULCONTROLELEMENT_IID \
  {0xbdc1d047, 0x6d22, 0x4813, \
    { 0xbc, 0x50, 0x63, 0x8c, 0xcb, 0x34, 0x9c, 0x7d }}

class NS_NO_VTABLE nsIDOMXULControlElement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULCONTROLELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULControlElement;

  /* attribute boolean disabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisabled(bool *aDisabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisabled(bool aDisabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULControlElement, NS_IDOMXULCONTROLELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULCONTROLELEMENT \
  NS_IMETHOD GetDisabled(bool *aDisabled) override; \
  NS_IMETHOD SetDisabled(bool aDisabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULCONTROLELEMENT \
  nsresult GetDisabled(bool *aDisabled); \
  nsresult SetDisabled(bool aDisabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULCONTROLELEMENT(_to) \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return _to GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return _to SetDisabled(aDisabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULCONTROLELEMENT(_to) \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisabled(aDisabled); } \
  NS_IMETHOD SetDisabled(bool aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisabled(aDisabled); } 


#endif /* __gen_nsIDOMXULControlElement_h__ */
