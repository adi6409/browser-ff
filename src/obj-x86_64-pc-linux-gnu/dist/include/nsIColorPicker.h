/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIColorPicker.idl
 */

#ifndef __gen_nsIColorPicker_h__
#define __gen_nsIColorPicker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIColorPickerShownCallback */
#define NS_ICOLORPICKERSHOWNCALLBACK_IID_STR "d2ce78d1-40b5-49d1-b66d-5801fcb9a385"

#define NS_ICOLORPICKERSHOWNCALLBACK_IID \
  {0xd2ce78d1, 0x40b5, 0x49d1, \
    { 0xb6, 0x6d, 0x58, 0x01, 0xfc, 0xb9, 0xa3, 0x85 }}

class NS_NO_VTABLE nsIColorPickerShownCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOLORPICKERSHOWNCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIColorPickerShownCallback;

  /* void update (in AString color); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Update(const nsAString& color) = 0;

  /* void done (in AString color); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Done(const nsAString& color) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIColorPickerShownCallback, NS_ICOLORPICKERSHOWNCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOLORPICKERSHOWNCALLBACK \
  NS_IMETHOD Update(const nsAString& color) override; \
  NS_IMETHOD Done(const nsAString& color) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOLORPICKERSHOWNCALLBACK \
  nsresult Update(const nsAString& color); \
  nsresult Done(const nsAString& color); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOLORPICKERSHOWNCALLBACK(_to) \
  NS_IMETHOD Update(const nsAString& color) override { return _to Update(color); } \
  NS_IMETHOD Done(const nsAString& color) override { return _to Done(color); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOLORPICKERSHOWNCALLBACK(_to) \
  NS_IMETHOD Update(const nsAString& color) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Update(color); } \
  NS_IMETHOD Done(const nsAString& color) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Done(color); } 


/* starting interface:    nsIColorPicker */
#define NS_ICOLORPICKER_IID_STR "de229d37-a8a6-46f1-969a-0c1de33d0ad7"

#define NS_ICOLORPICKER_IID \
  {0xde229d37, 0xa8a6, 0x46f1, \
    { 0x96, 0x9a, 0x0c, 0x1d, 0xe3, 0x3d, 0x0a, 0xd7 }}

class NS_NO_VTABLE nsIColorPicker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOLORPICKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIColorPicker;

  /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, const nsAString& initialColor) = 0;

  /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Open(nsIColorPickerShownCallback *aColorPickerShownCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIColorPicker, NS_ICOLORPICKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOLORPICKER \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, const nsAString& initialColor) override; \
  NS_IMETHOD Open(nsIColorPickerShownCallback *aColorPickerShownCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOLORPICKER \
  nsresult Init(mozIDOMWindowProxy *parent, const nsAString& title, const nsAString& initialColor); \
  nsresult Open(nsIColorPickerShownCallback *aColorPickerShownCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOLORPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, const nsAString& initialColor) override { return _to Init(parent, title, initialColor); } \
  NS_IMETHOD Open(nsIColorPickerShownCallback *aColorPickerShownCallback) override { return _to Open(aColorPickerShownCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOLORPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, const nsAString& initialColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(parent, title, initialColor); } \
  NS_IMETHOD Open(nsIColorPickerShownCallback *aColorPickerShownCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aColorPickerShownCallback); } 


#endif /* __gen_nsIColorPicker_h__ */
