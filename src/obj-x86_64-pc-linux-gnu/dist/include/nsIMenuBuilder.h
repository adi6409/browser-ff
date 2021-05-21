/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIMenuBuilder.idl
 */

#ifndef __gen_nsIMenuBuilder_h__
#define __gen_nsIMenuBuilder_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIMenuBuilder */
#define NS_IMENUBUILDER_IID_STR "93f4a48f-d043-4f45-97fd-9771ea1af976"

#define NS_IMENUBUILDER_IID \
  {0x93f4a48f, 0xd043, 0x4f45, \
    { 0x97, 0xfd, 0x97, 0x71, 0xea, 0x1a, 0xf9, 0x76 }}

class NS_NO_VTABLE nsIMenuBuilder : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMENUBUILDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMenuBuilder;

  /* void openContainer (in AString aLabel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenContainer(const nsAString& aLabel) = 0;

  /* void addItemFor (in Element aElement, in boolean aCanLoadIcon); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddItemFor(mozilla::dom::Element *aElement, bool aCanLoadIcon) = 0;

  /* void addSeparator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddSeparator(void) = 0;

  /* void undoAddSeparator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UndoAddSeparator(void) = 0;

  /* void closeContainer (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseContainer(void) = 0;

  /* AString toJSONString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToJSONString(nsAString& _retval) = 0;

  /* void click (in AString aGeneratedItemId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Click(const nsAString& aGeneratedItemId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMenuBuilder, NS_IMENUBUILDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMENUBUILDER \
  NS_IMETHOD OpenContainer(const nsAString& aLabel) override; \
  NS_IMETHOD AddItemFor(mozilla::dom::Element *aElement, bool aCanLoadIcon) override; \
  NS_IMETHOD AddSeparator(void) override; \
  NS_IMETHOD UndoAddSeparator(void) override; \
  NS_IMETHOD CloseContainer(void) override; \
  NS_IMETHOD ToJSONString(nsAString& _retval) override; \
  NS_IMETHOD Click(const nsAString& aGeneratedItemId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMENUBUILDER \
  nsresult OpenContainer(const nsAString& aLabel); \
  nsresult AddItemFor(mozilla::dom::Element *aElement, bool aCanLoadIcon); \
  nsresult AddSeparator(void); \
  nsresult UndoAddSeparator(void); \
  nsresult CloseContainer(void); \
  nsresult ToJSONString(nsAString& _retval); \
  nsresult Click(const nsAString& aGeneratedItemId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMENUBUILDER(_to) \
  NS_IMETHOD OpenContainer(const nsAString& aLabel) override { return _to OpenContainer(aLabel); } \
  NS_IMETHOD AddItemFor(mozilla::dom::Element *aElement, bool aCanLoadIcon) override { return _to AddItemFor(aElement, aCanLoadIcon); } \
  NS_IMETHOD AddSeparator(void) override { return _to AddSeparator(); } \
  NS_IMETHOD UndoAddSeparator(void) override { return _to UndoAddSeparator(); } \
  NS_IMETHOD CloseContainer(void) override { return _to CloseContainer(); } \
  NS_IMETHOD ToJSONString(nsAString& _retval) override { return _to ToJSONString(_retval); } \
  NS_IMETHOD Click(const nsAString& aGeneratedItemId) override { return _to Click(aGeneratedItemId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMENUBUILDER(_to) \
  NS_IMETHOD OpenContainer(const nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenContainer(aLabel); } \
  NS_IMETHOD AddItemFor(mozilla::dom::Element *aElement, bool aCanLoadIcon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddItemFor(aElement, aCanLoadIcon); } \
  NS_IMETHOD AddSeparator(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddSeparator(); } \
  NS_IMETHOD UndoAddSeparator(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UndoAddSeparator(); } \
  NS_IMETHOD CloseContainer(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseContainer(); } \
  NS_IMETHOD ToJSONString(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToJSONString(_retval); } \
  NS_IMETHOD Click(const nsAString& aGeneratedItemId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Click(aGeneratedItemId); } 


#endif /* __gen_nsIMenuBuilder_h__ */
