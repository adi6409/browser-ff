/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionListener.idl
 */

#ifndef __gen_nsITransactionListener_h__
#define __gen_nsITransactionListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITransaction; /* forward declaration */

class nsITransactionManager; /* forward declaration */


/* starting interface:    nsITransactionListener */
#define NS_ITRANSACTIONLISTENER_IID_STR "58e330c4-7b48-11d2-98b9-00805f297d89"

#define NS_ITRANSACTIONLISTENER_IID \
  {0x58e330c4, 0x7b48, 0x11d2, \
    { 0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89 }}

class NS_NO_VTABLE nsITransactionListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSACTIONLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransactionListener;

  /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillDo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) = 0;

  /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidDo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aDoResult) = 0;

  /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) = 0;

  /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aUndoResult) = 0;

  /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) = 0;

  /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aRedoResult) = 0;

  /* boolean willBeginBatch (in nsITransactionManager aManager); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillBeginBatch(nsITransactionManager *aManager, bool *_retval) = 0;

  /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidBeginBatch(nsITransactionManager *aManager, nsresult aResult) = 0;

  /* boolean willEndBatch (in nsITransactionManager aManager); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillEndBatch(nsITransactionManager *aManager, bool *_retval) = 0;

  /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidEndBatch(nsITransactionManager *aManager, nsresult aResult) = 0;

  /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool *_retval) = 0;

  /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool aDidMerge, nsresult aMergeResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransactionListener, NS_ITRANSACTIONLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSACTIONLISTENER \
  NS_IMETHOD WillDo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override; \
  NS_IMETHOD DidDo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aDoResult) override; \
  NS_IMETHOD WillUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override; \
  NS_IMETHOD DidUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aUndoResult) override; \
  NS_IMETHOD WillRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override; \
  NS_IMETHOD DidRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aRedoResult) override; \
  NS_IMETHOD WillBeginBatch(nsITransactionManager *aManager, bool *_retval) override; \
  NS_IMETHOD DidBeginBatch(nsITransactionManager *aManager, nsresult aResult) override; \
  NS_IMETHOD WillEndBatch(nsITransactionManager *aManager, bool *_retval) override; \
  NS_IMETHOD DidEndBatch(nsITransactionManager *aManager, nsresult aResult) override; \
  NS_IMETHOD WillMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool *_retval) override; \
  NS_IMETHOD DidMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool aDidMerge, nsresult aMergeResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSACTIONLISTENER \
  nsresult WillDo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval); \
  nsresult DidDo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aDoResult); \
  nsresult WillUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval); \
  nsresult DidUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aUndoResult); \
  nsresult WillRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval); \
  nsresult DidRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aRedoResult); \
  nsresult WillBeginBatch(nsITransactionManager *aManager, bool *_retval); \
  nsresult DidBeginBatch(nsITransactionManager *aManager, nsresult aResult); \
  nsresult WillEndBatch(nsITransactionManager *aManager, bool *_retval); \
  nsresult DidEndBatch(nsITransactionManager *aManager, nsresult aResult); \
  nsresult WillMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool *_retval); \
  nsresult DidMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool aDidMerge, nsresult aMergeResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSACTIONLISTENER(_to) \
  NS_IMETHOD WillDo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return _to WillDo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidDo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aDoResult) override { return _to DidDo(aManager, aTransaction, aDoResult); } \
  NS_IMETHOD WillUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return _to WillUndo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aUndoResult) override { return _to DidUndo(aManager, aTransaction, aUndoResult); } \
  NS_IMETHOD WillRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return _to WillRedo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aRedoResult) override { return _to DidRedo(aManager, aTransaction, aRedoResult); } \
  NS_IMETHOD WillBeginBatch(nsITransactionManager *aManager, bool *_retval) override { return _to WillBeginBatch(aManager, _retval); } \
  NS_IMETHOD DidBeginBatch(nsITransactionManager *aManager, nsresult aResult) override { return _to DidBeginBatch(aManager, aResult); } \
  NS_IMETHOD WillEndBatch(nsITransactionManager *aManager, bool *_retval) override { return _to WillEndBatch(aManager, _retval); } \
  NS_IMETHOD DidEndBatch(nsITransactionManager *aManager, nsresult aResult) override { return _to DidEndBatch(aManager, aResult); } \
  NS_IMETHOD WillMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool *_retval) override { return _to WillMerge(aManager, aTopTransaction, aTransactionToMerge, _retval); } \
  NS_IMETHOD DidMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool aDidMerge, nsresult aMergeResult) override { return _to DidMerge(aManager, aTopTransaction, aTransactionToMerge, aDidMerge, aMergeResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSACTIONLISTENER(_to) \
  NS_IMETHOD WillDo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillDo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidDo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aDoResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidDo(aManager, aTransaction, aDoResult); } \
  NS_IMETHOD WillUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillUndo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidUndo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aUndoResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidUndo(aManager, aTransaction, aUndoResult); } \
  NS_IMETHOD WillRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillRedo(aManager, aTransaction, _retval); } \
  NS_IMETHOD DidRedo(nsITransactionManager *aManager, nsITransaction *aTransaction, nsresult aRedoResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidRedo(aManager, aTransaction, aRedoResult); } \
  NS_IMETHOD WillBeginBatch(nsITransactionManager *aManager, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillBeginBatch(aManager, _retval); } \
  NS_IMETHOD DidBeginBatch(nsITransactionManager *aManager, nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidBeginBatch(aManager, aResult); } \
  NS_IMETHOD WillEndBatch(nsITransactionManager *aManager, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillEndBatch(aManager, _retval); } \
  NS_IMETHOD DidEndBatch(nsITransactionManager *aManager, nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidEndBatch(aManager, aResult); } \
  NS_IMETHOD WillMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillMerge(aManager, aTopTransaction, aTransactionToMerge, _retval); } \
  NS_IMETHOD DidMerge(nsITransactionManager *aManager, nsITransaction *aTopTransaction, nsITransaction *aTransactionToMerge, bool aDidMerge, nsresult aMergeResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidMerge(aManager, aTopTransaction, aTransactionToMerge, aDidMerge, aMergeResult); } 


#endif /* __gen_nsITransactionListener_h__ */
