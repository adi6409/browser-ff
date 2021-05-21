/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIQueryContentEventResult.idl
 */

#ifndef __gen_nsIQueryContentEventResult_h__
#define __gen_nsIQueryContentEventResult_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIQueryContentEventResult */
#define NS_IQUERYCONTENTEVENTRESULT_IID_STR "e2c39e0e-345f-451a-a7b2-e0230d555847"

#define NS_IQUERYCONTENTEVENTRESULT_IID \
  {0xe2c39e0e, 0x345f, 0x451a, \
    { 0xa7, 0xb2, 0xe0, 0x23, 0x0d, 0x55, 0x58, 0x47 }}

class NS_NO_VTABLE nsIQueryContentEventResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IQUERYCONTENTEVENTRESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIQueryContentEventResult;

  /* readonly attribute unsigned long offset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOffset(uint32_t *aOffset) = 0;

  /* readonly attribute unsigned long tentativeCaretOffset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTentativeCaretOffset(uint32_t *aTentativeCaretOffset) = 0;

  /* readonly attribute boolean reversed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReversed(bool *aReversed) = 0;

  /* readonly attribute long left; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLeft(int32_t *aLeft) = 0;

  /* readonly attribute long top; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTop(int32_t *aTop) = 0;

  /* readonly attribute long width; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWidth(int32_t *aWidth) = 0;

  /* readonly attribute long height; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHeight(int32_t *aHeight) = 0;

  /* readonly attribute AString text; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetText(nsAString& aText) = 0;

  /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCharacterRect(int32_t offset, int32_t *left, int32_t *top, int32_t *width, int32_t *height) = 0;

  /* readonly attribute boolean succeeded; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSucceeded(bool *aSucceeded) = 0;

  /* readonly attribute boolean notFound; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotFound(bool *aNotFound) = 0;

  /* readonly attribute boolean tentativeCaretOffsetNotFound; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTentativeCaretOffsetNotFound(bool *aTentativeCaretOffsetNotFound) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIQueryContentEventResult, NS_IQUERYCONTENTEVENTRESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIQUERYCONTENTEVENTRESULT \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override; \
  NS_IMETHOD GetTentativeCaretOffset(uint32_t *aTentativeCaretOffset) override; \
  NS_IMETHOD GetReversed(bool *aReversed) override; \
  NS_IMETHOD GetLeft(int32_t *aLeft) override; \
  NS_IMETHOD GetTop(int32_t *aTop) override; \
  NS_IMETHOD GetWidth(int32_t *aWidth) override; \
  NS_IMETHOD GetHeight(int32_t *aHeight) override; \
  NS_IMETHOD GetText(nsAString& aText) override; \
  NS_IMETHOD GetCharacterRect(int32_t offset, int32_t *left, int32_t *top, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override; \
  NS_IMETHOD GetNotFound(bool *aNotFound) override; \
  NS_IMETHOD GetTentativeCaretOffsetNotFound(bool *aTentativeCaretOffsetNotFound) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIQUERYCONTENTEVENTRESULT \
  nsresult GetOffset(uint32_t *aOffset); \
  nsresult GetTentativeCaretOffset(uint32_t *aTentativeCaretOffset); \
  nsresult GetReversed(bool *aReversed); \
  nsresult GetLeft(int32_t *aLeft); \
  nsresult GetTop(int32_t *aTop); \
  nsresult GetWidth(int32_t *aWidth); \
  nsresult GetHeight(int32_t *aHeight); \
  nsresult GetText(nsAString& aText); \
  nsresult GetCharacterRect(int32_t offset, int32_t *left, int32_t *top, int32_t *width, int32_t *height); \
  nsresult GetSucceeded(bool *aSucceeded); \
  nsresult GetNotFound(bool *aNotFound); \
  nsresult GetTentativeCaretOffsetNotFound(bool *aTentativeCaretOffsetNotFound); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIQUERYCONTENTEVENTRESULT(_to) \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override { return _to GetOffset(aOffset); } \
  NS_IMETHOD GetTentativeCaretOffset(uint32_t *aTentativeCaretOffset) override { return _to GetTentativeCaretOffset(aTentativeCaretOffset); } \
  NS_IMETHOD GetReversed(bool *aReversed) override { return _to GetReversed(aReversed); } \
  NS_IMETHOD GetLeft(int32_t *aLeft) override { return _to GetLeft(aLeft); } \
  NS_IMETHOD GetTop(int32_t *aTop) override { return _to GetTop(aTop); } \
  NS_IMETHOD GetWidth(int32_t *aWidth) override { return _to GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(int32_t *aHeight) override { return _to GetHeight(aHeight); } \
  NS_IMETHOD GetText(nsAString& aText) override { return _to GetText(aText); } \
  NS_IMETHOD GetCharacterRect(int32_t offset, int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return _to GetCharacterRect(offset, left, top, width, height); } \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override { return _to GetSucceeded(aSucceeded); } \
  NS_IMETHOD GetNotFound(bool *aNotFound) override { return _to GetNotFound(aNotFound); } \
  NS_IMETHOD GetTentativeCaretOffsetNotFound(bool *aTentativeCaretOffsetNotFound) override { return _to GetTentativeCaretOffsetNotFound(aTentativeCaretOffsetNotFound); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIQUERYCONTENTEVENTRESULT(_to) \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffset(aOffset); } \
  NS_IMETHOD GetTentativeCaretOffset(uint32_t *aTentativeCaretOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTentativeCaretOffset(aTentativeCaretOffset); } \
  NS_IMETHOD GetReversed(bool *aReversed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReversed(aReversed); } \
  NS_IMETHOD GetLeft(int32_t *aLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLeft(aLeft); } \
  NS_IMETHOD GetTop(int32_t *aTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTop(aTop); } \
  NS_IMETHOD GetWidth(int32_t *aWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(int32_t *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeight(aHeight); } \
  NS_IMETHOD GetText(nsAString& aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetText(aText); } \
  NS_IMETHOD GetCharacterRect(int32_t offset, int32_t *left, int32_t *top, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharacterRect(offset, left, top, width, height); } \
  NS_IMETHOD GetSucceeded(bool *aSucceeded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSucceeded(aSucceeded); } \
  NS_IMETHOD GetNotFound(bool *aNotFound) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotFound(aNotFound); } \
  NS_IMETHOD GetTentativeCaretOffsetNotFound(bool *aTentativeCaretOffsetNotFound) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTentativeCaretOffsetNotFound(aTentativeCaretOffsetNotFound); } 


#endif /* __gen_nsIQueryContentEventResult_h__ */
