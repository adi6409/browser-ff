/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLAbsPosEditor.idl
 */

#ifndef __gen_nsIHTMLAbsPosEditor_h__
#define __gen_nsIHTMLAbsPosEditor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIHTMLAbsPosEditor */
#define NS_IHTMLABSPOSEDITOR_IID_STR "91375f52-20e6-4757-9835-eb04fabe5498"

#define NS_IHTMLABSPOSEDITOR_IID \
  {0x91375f52, 0x20e6, 0x4757, \
    { 0x98, 0x35, 0xeb, 0x04, 0xfa, 0xbe, 0x54, 0x98 }}

class NS_NO_VTABLE nsIHTMLAbsPosEditor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTMLABSPOSEDITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHTMLAbsPosEditor;

  /* [can_run_script] attribute boolean absolutePositioningEnabled; */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsolutePositioningEnabled(bool *aAbsolutePositioningEnabled) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAbsolutePositioningEnabled(bool aAbsolutePositioningEnabled) = 0;

  /* attribute boolean snapToGridEnabled; */
  NS_IMETHOD GetSnapToGridEnabled(bool *aSnapToGridEnabled) = 0;
  NS_IMETHOD SetSnapToGridEnabled(bool aSnapToGridEnabled) = 0;

  /* attribute unsigned long gridSize; */
  NS_IMETHOD GetGridSize(uint32_t *aGridSize) = 0;
  NS_IMETHOD SetGridSize(uint32_t aGridSize) = 0;

  /* [can_run_script] void refreshGrabber (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshGrabber(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHTMLAbsPosEditor, NS_IHTMLABSPOSEDITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTMLABSPOSEDITOR \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsolutePositioningEnabled(bool *aAbsolutePositioningEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAbsolutePositioningEnabled(bool aAbsolutePositioningEnabled) override; \
  NS_IMETHOD GetSnapToGridEnabled(bool *aSnapToGridEnabled) override; \
  NS_IMETHOD SetSnapToGridEnabled(bool aSnapToGridEnabled) override; \
  NS_IMETHOD GetGridSize(uint32_t *aGridSize) override; \
  NS_IMETHOD SetGridSize(uint32_t aGridSize) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshGrabber(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTMLABSPOSEDITOR \
  MOZ_CAN_RUN_SCRIPT nsresult GetAbsolutePositioningEnabled(bool *aAbsolutePositioningEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult SetAbsolutePositioningEnabled(bool aAbsolutePositioningEnabled); \
  nsresult GetSnapToGridEnabled(bool *aSnapToGridEnabled); \
  nsresult SetSnapToGridEnabled(bool aSnapToGridEnabled); \
  nsresult GetGridSize(uint32_t *aGridSize); \
  nsresult SetGridSize(uint32_t aGridSize); \
  MOZ_CAN_RUN_SCRIPT nsresult RefreshGrabber(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTMLABSPOSEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsolutePositioningEnabled(bool *aAbsolutePositioningEnabled) override { return _to GetAbsolutePositioningEnabled(aAbsolutePositioningEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAbsolutePositioningEnabled(bool aAbsolutePositioningEnabled) override { return _to SetAbsolutePositioningEnabled(aAbsolutePositioningEnabled); } \
  NS_IMETHOD GetSnapToGridEnabled(bool *aSnapToGridEnabled) override { return _to GetSnapToGridEnabled(aSnapToGridEnabled); } \
  NS_IMETHOD SetSnapToGridEnabled(bool aSnapToGridEnabled) override { return _to SetSnapToGridEnabled(aSnapToGridEnabled); } \
  NS_IMETHOD GetGridSize(uint32_t *aGridSize) override { return _to GetGridSize(aGridSize); } \
  NS_IMETHOD SetGridSize(uint32_t aGridSize) override { return _to SetGridSize(aGridSize); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshGrabber(void) override { return _to RefreshGrabber(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTMLABSPOSEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsolutePositioningEnabled(bool *aAbsolutePositioningEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAbsolutePositioningEnabled(aAbsolutePositioningEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAbsolutePositioningEnabled(bool aAbsolutePositioningEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAbsolutePositioningEnabled(aAbsolutePositioningEnabled); } \
  NS_IMETHOD GetSnapToGridEnabled(bool *aSnapToGridEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSnapToGridEnabled(aSnapToGridEnabled); } \
  NS_IMETHOD SetSnapToGridEnabled(bool aSnapToGridEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSnapToGridEnabled(aSnapToGridEnabled); } \
  NS_IMETHOD GetGridSize(uint32_t *aGridSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGridSize(aGridSize); } \
  NS_IMETHOD SetGridSize(uint32_t aGridSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGridSize(aGridSize); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshGrabber(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshGrabber(); } 


#endif /* __gen_nsIHTMLAbsPosEditor_h__ */
