var std_Symbol = Symbol;
function List() {
    this.length = 0;
}
MakeConstructible(List, {__proto__: null});
function Record() {
    return std_Object_create(null);
}
MakeConstructible(Record, {});
function ToBoolean(v) {
    return !!v;
}
function ToNumber(v) {
    return +v;
}
function SameValueZero(x, y) {
    return x === y || (x !== x && y !== y);
}
function GetMethod(V, P) {
    ;;
    var func = V[P];
    if (func === undefined || func === null)
        return undefined;
    if (!IsCallable(func))
        ThrowTypeError(10, typeof func);
    return func;
}
function IsPropertyKey(argument) {
    var type = typeof argument;
    return type === "string" || type === "symbol";
}
function SpeciesConstructor(obj, defaultConstructor) {
    ;;
    var ctor = obj.constructor;
    if (ctor === undefined)
        return defaultConstructor;
    if (!IsObject(ctor))
        ThrowTypeError(48, "object's 'constructor' property");
    var s = ctor[std_species];
    if (s === undefined || s === null)
        return defaultConstructor;
    if (IsConstructor(s))
        return s;
    ThrowTypeError(11, "@@species property of object's constructor");
}
function GetTypeError(msg) {
    try {
        callFunction(std_Function_apply, ThrowTypeError, undefined, arguments);
    } catch (e) {
        return e;
    }
    ;;
}
function GetAggregateError(msg) {
    try {
        callFunction(std_Function_apply, ThrowAggregateError, undefined, arguments);
    } catch (e) {
        return e;
    }
    ;;
}
function GetInternalError(msg) {
    try {
        callFunction(std_Function_apply, ThrowInternalError, undefined, arguments);
    } catch (e) {
        return e;
    }
    ;;
}
function NullFunction() {}
function CopyDataProperties(target, source, excludedItems) {
    ;;
    ;;
    if (source === undefined || source === null)
        return;
    var from = ToObject(source);
    var keys = CopyDataPropertiesOrGetOwnKeys(target, from, excludedItems);
    if (keys === null)
        return;
    for (var index = 0; index < keys.length; index++) {
        var key = keys[index];
        if (!hasOwn(key, excludedItems) &&
            callFunction(std_Object_propertyIsEnumerable, from, key))
        {
            _DefineDataProperty(target, key, from[key]);
        }
    }
}
function CopyDataPropertiesUnfiltered(target, source) {
    ;;
    if (source === undefined || source === null)
        return;
    var from = ToObject(source);
    var keys = CopyDataPropertiesOrGetOwnKeys(target, from, null);
    if (keys === null)
        return;
    for (var index = 0; index < keys.length; index++) {
        var key = keys[index];
        if (callFunction(std_Object_propertyIsEnumerable, from, key))
            _DefineDataProperty(target, key, from[key]);
    }
}
function outer() {
    return function inner() {
        return "foo";
    };
}
function ArrayIndexOf(searchElement ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (len === 0)
        return -1;
    var n = arguments.length > 1 ? ToInteger(arguments[1]) : 0;
    if (n >= len)
        return -1;
    var k;
    if (n >= 0) {
        k = n;
    } else {
        k = len + n;
        if (k < 0)
            k = 0;
    }
    for (; k < len; k++) {
        if (k in O && O[k] === searchElement)
            return k;
    }
    return -1;
}
function ArrayLastIndexOf(searchElement ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (len === 0)
        return -1;
    var n = arguments.length > 1 ? ToInteger(arguments[1]) : len - 1;
    var k;
    if (n > len - 1)
        k = len - 1;
    else if (n < 0)
        k = len + n;
    else
        k = n;
    for (; k >= 0; k--) {
        if (k in O && O[k] === searchElement)
            return k;
    }
    return -1;
}
function ArrayEvery(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.every");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        if (k in O) {
            if (!callContentFunction(callbackfn, T, O[k], k, O))
                return false;
        }
    }
    return true;
}
function ArraySome(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.some");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        if (k in O) {
            if (callContentFunction(callbackfn, T, O[k], k, O))
                return true;
        }
    }
    return false;
}
function ArraySort(comparefn) {
    if (comparefn !== undefined) {
        if (!IsCallable(comparefn))
            ThrowTypeError(6);
    }
    var O = ToObject(this);
    if (callFunction(ArrayNativeSort, O, comparefn))
        return O;
    var len = ToLength(O.length);
    if (len <= 1)
      return O;
    var wrappedCompareFn = comparefn;
    comparefn = function(x, y) {
        if (x === undefined) {
            if (y === undefined)
                return 0;
           return 1;
        }
        if (y === undefined)
            return -1;
        var v = ToNumber(wrappedCompareFn(x, y));
        return v !== v ? 0 : v;
    };
    return MergeSort(O, len, comparefn);
}
function ArrayForEach(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.forEach");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        if (k in O) {
            callContentFunction(callbackfn, T, O[k], k, O);
        }
    }
    return void 0;
}
function ArrayMap(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.map");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    var A = ArraySpeciesCreate(O, len);
    for (var k = 0; k < len; k++) {
        if (k in O) {
            var mappedValue = callContentFunction(callbackfn, T, O[k], k, O);
            _DefineDataProperty(A, k, mappedValue);
        }
    }
    return A;
}
function ArrayFilter(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.filter");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    var A = ArraySpeciesCreate(O, 0);
    for (var k = 0, to = 0; k < len; k++) {
        if (k in O) {
            var kValue = O[k];
            var selected = callContentFunction(callbackfn, T, kValue, k, O);
            if (selected)
                _DefineDataProperty(A, to++, kValue);
        }
    }
    return A;
}
function ArrayReduce(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.reduce");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var k = 0;
    var accumulator;
    if (arguments.length > 1) {
        accumulator = arguments[1];
    } else {
        if (len === 0)
            throw ThrowTypeError(44);
        var kPresent = false;
        do {
            if (k in O) {
                kPresent = true;
                break;
            }
        } while (++k < len);
        if (!kPresent)
          throw ThrowTypeError(44);
        accumulator = O[k++];
    }
    for (; k < len; k++) {
        if (k in O) {
            accumulator = callbackfn(accumulator, O[k], k, O);
        }
    }
    return accumulator;
}
function ArrayReduceRight(callbackfn ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.reduce");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var k = len - 1;
    var accumulator;
    if (arguments.length > 1) {
        accumulator = arguments[1];
    } else {
        if (len === 0)
            throw ThrowTypeError(44);
        var kPresent = false;
        do {
            if (k in O) {
                kPresent = true;
                break;
            }
        } while (--k >= 0);
        if (!kPresent)
            throw ThrowTypeError(44);
        accumulator = O[k--];
    }
    for (; k >= 0; k--) {
        if (k in O) {
            accumulator = callbackfn(accumulator, O[k], k, O);
        }
    }
    return accumulator;
}
function ArrayFind(predicate ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.find");
    if (!IsCallable(predicate))
        ThrowTypeError(10, DecompileArg(0, predicate));
    var T = arguments.length > 1 ? arguments[1] : undefined;
    for (var k = 0; k < len; k++) {
        var kValue = O[k];
        if (callContentFunction(predicate, T, kValue, k, O))
            return kValue;
    }
    return undefined;
}
function ArrayFindIndex(predicate ) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "Array.prototype.find");
    if (!IsCallable(predicate))
        ThrowTypeError(10, DecompileArg(0, predicate));
    var T = arguments.length > 1 ? arguments[1] : undefined;
    for (var k = 0; k < len; k++) {
        if (callContentFunction(predicate, T, O[k], k, O))
            return k;
    }
    return -1;
}
function ArrayCopyWithin(target, start, end = undefined) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    var relativeTarget = ToInteger(target);
    var to = relativeTarget < 0 ? std_Math_max(len + relativeTarget, 0)
                                : std_Math_min(relativeTarget, len);
    var relativeStart = ToInteger(start);
    var from = relativeStart < 0 ? std_Math_max(len + relativeStart, 0)
                                 : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0 ? std_Math_max(len + relativeEnd, 0)
                                : std_Math_min(relativeEnd, len);
    var count = std_Math_min(final - from, len - to);
    if (from < to && to < (from + count)) {
        from = from + count - 1;
        to = to + count - 1;
        while (count > 0) {
            if (from in O)
                O[to] = O[from];
            else
                delete O[to];
            from--;
            to--;
            count--;
        }
    } else {
        while (count > 0) {
            if (from in O)
                O[to] = O[from];
            else
                delete O[to];
            from++;
            to++;
            count--;
        }
    }
    return O;
}
function ArrayFill(value, start = 0, end = undefined) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    var relativeStart = ToInteger(start);
    var k = relativeStart < 0
            ? std_Math_max(len + relativeStart, 0)
            : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0
                ? std_Math_max(len + relativeEnd, 0)
                : std_Math_min(relativeEnd, len);
    for (; k < final; k++) {
        O[k] = value;
    }
    return O;
}
function ArrayIncludes(searchElement, fromIndex = 0) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    if (len === 0)
        return false;
    var n = ToInteger(fromIndex);
    var k;
    if (n >= 0) {
        k = n;
    } else {
        k = len + n;
        if (k < 0)
            k = 0;
    }
    while (k < len) {
        if (SameValueZero(searchElement, O[k]))
            return true;
        k++;
    }
    return false;
}
function CreateArrayIterator(obj, kind) {
    var iteratedObject = ToObject(obj);
    var iterator = NewArrayIterator();
    UnsafeSetReservedSlot(iterator, 0, iteratedObject);
    UnsafeSetReservedSlot(iterator, 1, 0);
    UnsafeSetReservedSlot(iterator, 2, kind);
    return iterator;
}
function ArrayIteratorNext() {
    var obj;
    if (!IsObject(this) || (obj = GuardToArrayIterator(this)) === null) {
        return callFunction(CallArrayIteratorMethodIfWrapped, this,
                            "ArrayIteratorNext");
    }
    var a = UnsafeGetReservedSlot(obj, 0);
    var result = { value: undefined, done: false };
    if (a === null) {
      result.done = true;
      return result;
    }
    var index = UnsafeGetReservedSlot(obj, 1);
    var itemKind = UnsafeGetInt32FromReservedSlot(obj, 2);
    var len;
    if (IsPossiblyWrappedTypedArray(a)) {
        len = PossiblyWrappedTypedArrayLength(a);
        if (len === 0) {
            if (PossiblyWrappedTypedArrayHasDetachedBuffer(a))
                ThrowTypeError(530);
        }
    } else {
        len = ToLength(a.length);
    }
    if (index >= len) {
        UnsafeSetReservedSlot(obj, 0, null);
        result.done = true;
        return result;
    }
    UnsafeSetReservedSlot(obj, 1, index + 1);
    if (itemKind === 1) {
        result.value = a[index];
        return result;
    }
    if (itemKind === 2) {
        var pair = [index, a[index]];
        result.value = pair;
        return result;
    }
    ;;
    result.value = index;
    return result;
}
function $ArrayValues() {
    return CreateArrayIterator(this, 1);
}
_SetCanonicalName($ArrayValues, "values");
function ArrayEntries() {
    return CreateArrayIterator(this, 2);
}
function ArrayKeys() {
    return CreateArrayIterator(this, 0);
}
function ArrayFrom(items, mapfn = undefined, thisArg = undefined) {
    var C = this;
    var mapping = mapfn !== undefined;
    if (mapping && !IsCallable(mapfn))
        ThrowTypeError(10, DecompileArg(1, mapfn));
    var T = thisArg;
    var usingIterator = items[std_iterator];
    if (usingIterator !== undefined && usingIterator !== null) {
        if (!IsCallable(usingIterator))
            ThrowTypeError(65, DecompileArg(0, items));
        var A = IsConstructor(C) ? new C() : [];
        var iterator = MakeIteratorWrapper(items, usingIterator);
        var k = 0;
        for (var nextValue of allowContentIter(iterator)) {
            var mappedValue = mapping ? callContentFunction(mapfn, T, nextValue, k) : nextValue;
            _DefineDataProperty(A, k++, mappedValue);
        }
        A.length = k;
        return A;
    }
    var arrayLike = ToObject(items);
    var len = ToLength(arrayLike.length);
    var A = IsConstructor(C) ? new C(len) : std_Array(len);
    for (var k = 0; k < len; k++) {
        var kValue = items[k];
        var mappedValue = mapping ? callContentFunction(mapfn, T, kValue, k) : kValue;
        _DefineDataProperty(A, k, mappedValue);
    }
    A.length = len;
    return A;
}
function MakeIteratorWrapper(items, method) {
    ;;
    return {
        [std_iterator]: function IteratorMethod() {
            return callContentFunction(method, items);
        },
    };
}
function ArrayToString() {
    var array = ToObject(this);
    var func = array.join;
    if (!IsCallable(func))
        return callFunction(std_Object_toString, array);
    return callContentFunction(func, array);
}
function ArrayToLocaleString(locales, options) {
    ;;
    var array = this;
    var len = ToLength(array.length);
    if (len === 0)
        return "";
    var firstElement = array[0];
    var R;
    if (firstElement === undefined || firstElement === null) {
        R = "";
    } else {
        R = ToString(callContentFunction(firstElement.toLocaleString, firstElement, locales, options));
    }
    var separator = ",";
    for (var k = 1; k < len; k++) {
        var nextElement = array[k];
        R += separator;
        if (!(nextElement === undefined || nextElement === null)) {
            R += ToString(callContentFunction(nextElement.toLocaleString, nextElement, locales, options));
        }
    }
    return R;
}
function $ArraySpecies() {
    return this;
}
_SetCanonicalName($ArraySpecies, "get [Symbol.species]");
function ArraySpeciesCreate(originalArray, length) {
    ;;
    ;;
    if (length === -0)
        length = 0;
    if (!IsArray(originalArray))
        return std_Array(length);
    var C = originalArray.constructor;
    if (IsConstructor(C) && IsCrossRealmArrayConstructor(C))
        return std_Array(length);
    if (IsObject(C)) {
        C = C[std_species];
        if (C === GetBuiltinConstructor("Array"))
            return std_Array(length);
        if (C === null)
            return std_Array(length);
    }
    if (C === undefined)
        return std_Array(length);
    if (!IsConstructor(C))
        ThrowTypeError(11, "constructor property");
    return new C(length);
}
function IsConcatSpreadable(O) {
    if (!IsObject(O))
        return false;
    var spreadable = O[std_isConcatSpreadable];
    if (spreadable !== undefined)
        return ToBoolean(spreadable);
    return IsArray(O);
}
function ArrayConcat(arg1) {
    var O = ToObject(this);
    var A = ArraySpeciesCreate(O, 0);
    var n = 0;
    var i = 0, argsLen = arguments.length;
    var E = O;
    var k, len;
    while (true) {
        if (IsConcatSpreadable(E)) {
            len = ToLength(E.length);
            if (n + len > 0x1fffffffffffff)
                ThrowTypeError(522);
            if (IsPackedArray(A) && IsPackedArray(E)) {
                for (k = 0; k < len; k++) {
                    _DefineDataProperty(A, n, E[k]);
                    n++;
                }
            } else {
                for (k = 0; k < len; k++) {
                    if (k in E)
                        _DefineDataProperty(A, n, E[k]);
                    n++;
                }
            }
        } else {
            if (n >= 0x1fffffffffffff)
                ThrowTypeError(522);
            _DefineDataProperty(A, n, E);
            n++;
        }
        if (i >= argsLen)
            break;
        E = arguments[i];
        i++;
    }
    A.length = n;
    return A;
}
function ArrayFlatMap(mapperFunction ) {
    var O = ToObject(this);
    var sourceLen = ToLength(O.length);
    if (!IsCallable(mapperFunction))
        ThrowTypeError(10, DecompileArg(0, mapperFunction));
    var T = arguments.length > 1 ? arguments[1] : undefined;
    var A = ArraySpeciesCreate(O, 0);
    FlattenIntoArray(A, O, sourceLen, 0, 1, mapperFunction, T);
    return A;
}
function ArrayFlat( ) {
    var O = ToObject(this);
    var sourceLen = ToLength(O.length);
    var depthNum = 1;
    if (arguments.length > 0 && arguments[0] !== undefined)
        depthNum = ToInteger(arguments[0]);
    var A = ArraySpeciesCreate(O, 0);
    FlattenIntoArray(A, O, sourceLen, 0, depthNum);
    return A;
}
function FlattenIntoArray(target, source, sourceLen, start, depth, mapperFunction, thisArg) {
    var targetIndex = start;
    for (var sourceIndex = 0; sourceIndex < sourceLen; sourceIndex++) {
        if (sourceIndex in source) {
            var element = source[sourceIndex];
            if (mapperFunction) {
                ;;
                element = callContentFunction(mapperFunction, thisArg, element, sourceIndex, source);
            }
            var shouldFlatten = false;
            if (depth > 0) {
                shouldFlatten = IsArray(element);
            }
            if (shouldFlatten) {
                var elementLen = ToLength(element.length);
                targetIndex = FlattenIntoArray(target, element, elementLen, targetIndex, depth - 1);
            } else {
                if (targetIndex >= 0x1fffffffffffff)
                    ThrowTypeError(522);
                _DefineDataProperty(target, targetIndex, element);
                targetIndex++;
            }
        }
    }
    return targetIndex;
}
function ArrayAt(index) {
    var O = ToObject(this);
    var len = ToLength(O.length);
    var relativeIndex = ToInteger(index);
    var k;
    if (relativeIndex >= 0) {
        k = relativeIndex;
    } else {
        k = len + relativeIndex;
    }
    if (k < 0 || k >= len) {
        return undefined;
    }
    return O[k];
}
_SetIsInlinableLargeFunction(ArrayAt);
function AsyncFunctionNext(val) {
    ;;
    return resumeGenerator(this, val, "next");
}
function AsyncFunctionThrow(val) {
    ;;
    return resumeGenerator(this, val, "throw");
}
function AsyncIteratorIdentity() {
    return this;
}
function AsyncGeneratorNext(val) {
    ;;
    return resumeGenerator(this, val, "next");
}
function AsyncGeneratorThrow(val) {
    ;;
    return resumeGenerator(this, val, "throw");
}
function AsyncGeneratorReturn(val) {
    ;;
    return resumeGenerator(this, val, "return");
}
async function AsyncIteratorClose(iteratorRecord, value) {
  const iterator = iteratorRecord.iterator;
  const returnMethod = iterator.return;
  if (returnMethod !== undefined && returnMethod !== null) {
    const result = await callContentFunction(returnMethod, iterator);
    if (!IsObject(result)) {
      ThrowTypeError(48, DecompileArg(0, result));
    }
  }
  return value;
}
function GetAsyncIteratorDirectWrapper(obj) {
  if (!IsObject(obj)) {
    ThrowTypeError(48, obj);
  }
  const nextMethod = obj.next;
  if (!IsCallable(nextMethod)) {
    ThrowTypeError(10, nextMethod);
  }
  return {
    [std_asyncIterator]: function AsyncIteratorMethod() {
      return this;
    },
    next(value) {
      return callContentFunction(nextMethod, obj, value);
    },
    async return(value) {
      const returnMethod = obj.return;
      if (returnMethod !== undefined && returnMethod !== null) {
        return callContentFunction(returnMethod, obj, value);
      }
      return {done: true, value};
    },
  };
}
function AsyncIteratorHelperNext(value) {
  let O;
  if (!IsObject(this) || (O = GuardToAsyncIteratorHelper(this)) === null) {
    return callFunction(CallAsyncIteratorHelperMethodIfWrapped, this,
                        value, "AsyncIteratorHelperNext");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callFunction(IntrinsicAsyncGeneratorNext, generator, value);
}
function AsyncIteratorHelperReturn(value) {
  let O;
  if (!IsObject(this) || (O = GuardToAsyncIteratorHelper(this)) === null) {
    return callFunction(CallAsyncIteratorHelperMethodIfWrapped, this,
                        value, "AsyncIteratorHelperReturn");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callFunction(IntrinsicAsyncGeneratorReturn, generator, value);
}
function AsyncIteratorHelperThrow(value) {
  let O;
  if (!IsObject(this) || (O = GuardToAsyncIteratorHelper(this)) === null) {
    return callFunction(CallAsyncIteratorHelperMethodIfWrapped, this,
                        value, "AsyncIteratorHelperThrow");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callFunction(IntrinsicAsyncGeneratorThrow, generator, value);
}
function AsyncIteratorMap(mapper) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(mapper)) {
    ThrowTypeError(10, DecompileArg(0, mapper));
  }
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorMapGenerator(iterated, mapper);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorMapGenerator(iterated, mapper) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = await IteratorNext(iterated, lastValue);
        !next.done;
        next = await IteratorNext(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      lastValue = yield callContentFunction(mapper, undefined, value);
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated);
    }
  }
}
function AsyncIteratorFilter(filterer) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(filterer)) {
    ThrowTypeError(10, DecompileArg(0, filterer));
  }
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorFilterGenerator(iterated, filterer);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorFilterGenerator(iterated, filterer) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = await IteratorNext(iterated, lastValue);
        !next.done;
        next = await IteratorNext(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      if (await callContentFunction(filterer, undefined, value)) {
        lastValue = yield value;
      }
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated);
    }
  }
}
function AsyncIteratorTake(limit) {
  const iterated = GetIteratorDirect(this);
  const remaining = ToInteger(limit);
  if (remaining < 0) {
    ThrowRangeError(641);
  }
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorTakeGenerator(iterated, remaining);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorTakeGenerator(iterated, remaining) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (; remaining > 0; remaining--) {
      const next = await IteratorNext(iterated, lastValue);
      if (next.done) {
        return undefined;
      }
      const value = next.value;
      needClose = true;
      lastValue = yield value;
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated, undefined);
    }
  }
  return AsyncIteratorClose(iterated, undefined);
}
function AsyncIteratorDrop(limit) {
  const iterated = GetIteratorDirect(this);
  const remaining = ToInteger(limit);
  if (remaining < 0) {
    ThrowRangeError(641);
  }
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorDropGenerator(iterated, remaining);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorDropGenerator(iterated, remaining) {
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (; remaining > 0; remaining--) {
      const next = await IteratorNext(iterated);
      if (next.done) {
        return;
      }
    }
    let lastValue;
    for (let next = await IteratorNext(iterated, lastValue);
        !next.done;
        next = await IteratorNext(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      lastValue = yield value;
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated);
    }
  }
}
function AsyncIteratorAsIndexedPairs() {
  const iterated = GetIteratorDirect(this);
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorAsIndexedPairsGenerator(iterated);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorAsIndexedPairsGenerator(iterated) {
  let needClose = true;
  try {
    yield;
    needClose = false;
    let lastValue;
    for (let next = await IteratorNext(iterated, lastValue), index = 0;
        !next.done;
        next = await IteratorNext(iterated, lastValue), index++) {
      const value = next.value;
      needClose = true;
      lastValue = yield [index, value];
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated);
    }
  }
}
function AsyncIteratorFlatMap(mapper) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(mapper)) {
    ThrowTypeError(10, DecompileArg(0, mapper));
  }
  const iteratorHelper = NewAsyncIteratorHelper();
  const generator = AsyncIteratorFlatMapGenerator(iterated, mapper);
  callFunction(IntrinsicAsyncGeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
async function* AsyncIteratorFlatMapGenerator(iterated, mapper) {
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = await IteratorNext(iterated);
        !next.done;
        next = await IteratorNext(iterated)) {
      const value = next.value;
      needClose = true;
      const mapped = await callContentFunction(mapper, undefined, value);
      for await (const innerValue of allowContentIter(mapped)) {
        yield innerValue;
      }
      needClose = false;
    }
  } finally {
    if (needClose) {
      AsyncIteratorClose(iterated);
    }
  }
}
async function AsyncIteratorReduce(reducer ) {
  const iterated = GetAsyncIteratorDirectWrapper(this);
  if (!IsCallable(reducer)) {
    ThrowTypeError(10, DecompileArg(0, reducer));
  }
  let accumulator;
  if (arguments.length === 1) {
    const next = await callContentFunction(iterated.next, iterated);
    if (!IsObject(next)) {
      ThrowTypeError(48, DecompileArg(0, next));
    }
    if (next.done) {
      ThrowTypeError(45);
    }
    accumulator = next.value;
  } else {
    accumulator = arguments[1];
  }
  for await (const value of allowContentIter(iterated)) {
    accumulator = await callContentFunction(reducer, undefined, accumulator, value);
  }
  return accumulator;
}
async function AsyncIteratorToArray() {
  const iterated = {[std_asyncIterator]: () => this};
  const items = [];
  let index = 0;
  for await (const value of allowContentIter(iterated)) {
    _DefineDataProperty(items, index++, value);
  }
  return items;
}
async function AsyncIteratorForEach(fn) {
  const iterated = GetAsyncIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for await (const value of allowContentIter(iterated)) {
    await callContentFunction(fn, undefined, value);
  }
}
async function AsyncIteratorSome(fn) {
  const iterated = GetAsyncIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for await (const value of allowContentIter(iterated)) {
    if (await callContentFunction(fn, undefined, value)) {
      return true;
    }
  }
  return false;
}
async function AsyncIteratorEvery(fn) {
  const iterated = GetAsyncIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for await (const value of allowContentIter(iterated)) {
    if (!await callContentFunction(fn, undefined, value)) {
      return false;
    }
  }
  return true;
}
async function AsyncIteratorFind(fn) {
  const iterated = GetAsyncIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for await (const value of allowContentIter(iterated)) {
    if (await callContentFunction(fn, undefined, value)) {
      return value;
    }
  }
}
function BigInt_toLocaleString() {
    var x = callFunction(std_BigInt_valueOf, this);
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var numberFormat;
    if (locales === undefined && options === undefined) {
        if (!IsRuntimeDefaultLocale(numberFormatCache.runtimeDefaultLocale)) {
            numberFormatCache.numberFormat = intl_NumberFormat(locales, options);
            numberFormatCache.runtimeDefaultLocale = RuntimeDefaultLocale();
        }
        numberFormat = numberFormatCache.numberFormat;
    } else {
        numberFormat = intl_NumberFormat(locales, options);
    }
    return intl_FormatNumber(numberFormat, x, false, false);
}
var dateTimeFormatCache = new Record();
function GetCachedFormat(format, required, defaults) {
    ;;
    var formatters;
    if (!IsRuntimeDefaultLocale(dateTimeFormatCache.runtimeDefaultLocale) ||
        !intl_isDefaultTimeZone(dateTimeFormatCache.icuDefaultTimeZone))
    {
        formatters = dateTimeFormatCache.formatters = new Record();
        dateTimeFormatCache.runtimeDefaultLocale = RuntimeDefaultLocale();
        dateTimeFormatCache.icuDefaultTimeZone = intl_defaultTimeZone();
    } else {
        formatters = dateTimeFormatCache.formatters;
    }
    var fmt = formatters[format];
    if (fmt === undefined) {
        var options = ToDateTimeOptions(undefined, required, defaults);
        fmt = formatters[format] = intl_DateTimeFormat(undefined, options);
    }
    return fmt;
}
function Date_toLocaleString() {
    var x = callFunction(ThisTimeValue, this, 2);
    if (Number_isNaN(x))
        return "Invalid Date";
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var dateTimeFormat;
    if (locales === undefined && options === undefined) {
        dateTimeFormat = GetCachedFormat("dateTimeFormat", "any", "all");
    } else {
        options = ToDateTimeOptions(options, "any", "all");
        dateTimeFormat = intl_DateTimeFormat(locales, options);
    }
    return intl_FormatDateTime(dateTimeFormat, x, false);
}
function Date_toLocaleDateString() {
    var x = callFunction(ThisTimeValue, this, 1);
    if (Number_isNaN(x))
        return "Invalid Date";
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var dateTimeFormat;
    if (locales === undefined && options === undefined) {
        dateTimeFormat = GetCachedFormat("dateFormat", "date", "date");
    } else {
        options = ToDateTimeOptions(options, "date", "date");
        dateTimeFormat = intl_DateTimeFormat(locales, options);
    }
    return intl_FormatDateTime(dateTimeFormat, x, false);
}
function Date_toLocaleTimeString() {
    var x = callFunction(ThisTimeValue, this, 0);
    if (Number_isNaN(x))
        return "Invalid Date";
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var dateTimeFormat;
    if (locales === undefined && options === undefined) {
        dateTimeFormat = GetCachedFormat("timeFormat", "time", "time");
    } else {
        options = ToDateTimeOptions(options, "time", "time");
        dateTimeFormat = intl_DateTimeFormat(locales, options);
    }
    return intl_FormatDateTime(dateTimeFormat, x, false);
}
function ErrorToString()
{
  var obj = this;
  if (!IsObject(obj))
    ThrowTypeError(3, "Error", "toString", "value");
  var name = obj.name;
  name = (name === undefined) ? "Error" : ToString(name);
  var msg = obj.message;
  msg = (msg === undefined) ? "" : ToString(msg);
  if (name === "")
    return msg;
  if (msg === "")
    return name;
  return name + ": " + msg;
}
function ErrorToStringWithTrailingNewline()
{
  return callFunction(std_Function_apply, ErrorToString, this, []) + "\n";
}
function FunctionBind(thisArg, ...boundArgs) {
    var target = this;
    if (!IsCallable(target))
        ThrowTypeError(3, "Function", "bind", target);
    var F;
    var argCount = boundArgs.length;
    switch (argCount) {
      case 0:
        F = bind_bindFunction0(target, thisArg, boundArgs);
        break;
      case 1:
        F = bind_bindFunction1(target, thisArg, boundArgs);
        break;
      case 2:
        F = bind_bindFunction2(target, thisArg, boundArgs);
        break;
      default:
        F = bind_bindFunctionN(target, thisArg, boundArgs);
    }
    _FinishBoundFunctionInit(F, target, argCount);
    void std_Function_apply;
    return F;
}
function bind_bindFunction0(fun, thisArg, boundArgs) {
    return function bound() {
        if (false) void boundArgs;
        var newTarget;
        if (_IsConstructing()) {
            newTarget = new.target;
            if (newTarget === bound)
                newTarget = fun;
            switch (arguments.length) {
              case 0:
                return constructContentFunction(fun, newTarget);
              case 1:
                return constructContentFunction(fun, newTarget, arguments[0]);
              case 2:
                return constructContentFunction(fun, newTarget, arguments[0], arguments[1]);
              case 3:
                return constructContentFunction(fun, newTarget, arguments[0], arguments[1], arguments[2]);
              case 4:
                return constructContentFunction(fun, newTarget, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return constructContentFunction(fun, newTarget, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
              default:
                var args = callFunction(std_Function_apply, bind_mapArguments, null, arguments);
                return bind_constructFunctionN(fun, newTarget, args);
            }
        } else {
            switch (arguments.length) {
              case 0:
                return callContentFunction(fun, thisArg);
              case 1:
                return callContentFunction(fun, thisArg, arguments[0]);
              case 2:
                return callContentFunction(fun, thisArg, arguments[0], arguments[1]);
              case 3:
                return callContentFunction(fun, thisArg, arguments[0], arguments[1], arguments[2]);
              case 4:
                return callContentFunction(fun, thisArg, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return callContentFunction(fun, thisArg, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
              default:
                return callFunction(std_Function_apply, fun, thisArg, arguments);
            }
        }
    };
}
function bind_bindFunction1(fun, thisArg, boundArgs) {
    var bound1 = boundArgs[0];
    var combiner = null;
    return function bound() {
        if (false) void boundArgs;
        var newTarget;
        if (_IsConstructing()) {
            newTarget = new.target;
            if (newTarget === bound)
                newTarget = fun;
            switch (arguments.length) {
              case 0:
                return constructContentFunction(fun, newTarget, bound1);
              case 1:
                return constructContentFunction(fun, newTarget, bound1, arguments[0]);
              case 2:
                return constructContentFunction(fun, newTarget, bound1, arguments[0], arguments[1]);
              case 3:
                return constructContentFunction(fun, newTarget, bound1, arguments[0], arguments[1], arguments[2]);
              case 4:
                return constructContentFunction(fun, newTarget, bound1, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return constructContentFunction(fun, newTarget, bound1, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
            }
        } else {
            switch (arguments.length) {
              case 0:
                return callContentFunction(fun, thisArg, bound1);
              case 1:
                return callContentFunction(fun, thisArg, bound1, arguments[0]);
              case 2:
                return callContentFunction(fun, thisArg, bound1, arguments[0], arguments[1]);
              case 3:
                return callContentFunction(fun, thisArg, bound1, arguments[0], arguments[1], arguments[2]);
              case 4:
                return callContentFunction(fun, thisArg, bound1, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return callContentFunction(fun, thisArg, bound1, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
            }
        }
        if (combiner === null) {
            combiner = function() {
                var callArgsCount = arguments.length;
                var args = std_Array(1 + callArgsCount);
                _DefineDataProperty(args, 0, bound1);
                for (var i = 0; i < callArgsCount; i++)
                    _DefineDataProperty(args, i + 1, arguments[i]);
                return args;
            };
        }
        var args = callFunction(std_Function_apply, combiner, null, arguments);
        if (newTarget === undefined)
            return bind_applyFunctionN(fun, thisArg, args);
        return bind_constructFunctionN(fun, newTarget, args);
    };
}
function bind_bindFunction2(fun, thisArg, boundArgs) {
    var bound1 = boundArgs[0];
    var bound2 = boundArgs[1];
    var combiner = null;
    return function bound() {
        if (false) void boundArgs;
        var newTarget;
        if (_IsConstructing()) {
            newTarget = new.target;
            if (newTarget === bound)
                newTarget = fun;
            switch (arguments.length) {
              case 0:
                return constructContentFunction(fun, newTarget, bound1, bound2);
              case 1:
                return constructContentFunction(fun, newTarget, bound1, bound2, arguments[0]);
              case 2:
                return constructContentFunction(fun, newTarget, bound1, bound2, arguments[0], arguments[1]);
              case 3:
                return constructContentFunction(fun, newTarget, bound1, bound2, arguments[0], arguments[1], arguments[2]);
              case 4:
                return constructContentFunction(fun, newTarget, bound1, bound2, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return constructContentFunction(fun, newTarget, bound1, bound2, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
            }
        } else {
            switch (arguments.length) {
              case 0:
                return callContentFunction(fun, thisArg, bound1, bound2);
              case 1:
                return callContentFunction(fun, thisArg, bound1, bound2, arguments[0]);
              case 2:
                return callContentFunction(fun, thisArg, bound1, bound2, arguments[0], arguments[1]);
              case 3:
                return callContentFunction(fun, thisArg, bound1, bound2, arguments[0], arguments[1], arguments[2]);
              case 4:
                return callContentFunction(fun, thisArg, bound1, bound2, arguments[0], arguments[1], arguments[2], arguments[3]);
              case 5:
                return callContentFunction(fun, thisArg, bound1, bound2, arguments[0], arguments[1], arguments[2], arguments[3], arguments[4]);
            }
        }
        if (combiner === null) {
            combiner = function() {
                var callArgsCount = arguments.length;
                var args = std_Array(2 + callArgsCount);
                _DefineDataProperty(args, 0, bound1);
                _DefineDataProperty(args, 1, bound2);
                for (var i = 0; i < callArgsCount; i++)
                    _DefineDataProperty(args, i + 2, arguments[i]);
                return args;
            };
        }
        var args = callFunction(std_Function_apply, combiner, null, arguments);
        if (newTarget === undefined)
            return bind_applyFunctionN(fun, thisArg, args);
        return bind_constructFunctionN(fun, newTarget, args);
    };
}
function bind_bindFunctionN(fun, thisArg, boundArgs) {
    ;;
    var combiner = null;
    return function bound() {
        var newTarget;
        if (_IsConstructing()) {
            newTarget = new.target;
            if (newTarget === bound)
                newTarget = fun;
        }
        if (arguments.length === 0) {
            if (newTarget !== undefined)
                return bind_constructFunctionN(fun, newTarget, boundArgs);
            return bind_applyFunctionN(fun, thisArg, boundArgs);
        }
        if (combiner === null) {
            combiner = function() {
                var boundArgsCount = boundArgs.length;
                var callArgsCount = arguments.length;
                var args = std_Array(boundArgsCount + callArgsCount);
                for (var i = 0; i < boundArgsCount; i++)
                    _DefineDataProperty(args, i, boundArgs[i]);
                for (var i = 0; i < callArgsCount; i++)
                    _DefineDataProperty(args, i + boundArgsCount, arguments[i]);
                return args;
            };
        }
        var args = callFunction(std_Function_apply, combiner, null, arguments);
        if (newTarget !== undefined)
            return bind_constructFunctionN(fun, newTarget, args);
        return bind_applyFunctionN(fun, thisArg, args);
    };
}
function bind_mapArguments() {
    var len = arguments.length;
    var args = std_Array(len);
    for (var i = 0; i < len; i++)
        _DefineDataProperty(args, i, arguments[i]);
    return args;
}
function bind_applyFunctionN(fun, thisArg, args) {
    switch (args.length) {
      case 0:
        return callContentFunction(fun, thisArg);
      case 1:
        return callContentFunction(fun, thisArg, args[0]);
      case 2:
        return callContentFunction(fun, thisArg, args[0], args[1]);
      case 3:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2]);
      case 4:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3]);
      case 5:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3], args[4]);
      case 6:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3], args[4], args[5]);
      case 7:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
      case 8:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
      case 9:
        return callContentFunction(fun, thisArg, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
      default:
        return callFunction(std_Function_apply, fun, thisArg, args);
    }
}
function bind_constructFunctionN(fun, newTarget, args) {
    switch (args.length) {
      case 1:
        return constructContentFunction(fun, newTarget, args[0]);
      case 2:
        return constructContentFunction(fun, newTarget, args[0], args[1]);
      case 3:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2]);
      case 4:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3]);
      case 5:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4]);
      case 6:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5]);
      case 7:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
      case 8:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
      case 9:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
      case 10:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
      case 11:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]);
      case 12:
        return constructContentFunction(fun, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]);
      default:
        ;;
        return _ConstructFunction(fun, newTarget, args);
    }
}
function GeneratorNext(val) {
    if (!IsSuspendedGenerator(this)) {
        if (!IsObject(this) || !IsGeneratorObject(this))
            return callFunction(CallGeneratorMethodIfWrapped, this, val, "GeneratorNext");
        if (GeneratorObjectIsClosed(this))
            return { value: undefined, done: true };
        if (GeneratorIsRunning(this))
            ThrowTypeError(38);
    }
    try {
        return resumeGenerator(this, val, "next");
    } catch (e) {
        if (!GeneratorObjectIsClosed(this))
            GeneratorSetClosed(this);
        throw e;
    }
}
function GeneratorThrow(val) {
    if (!IsSuspendedGenerator(this)) {
        if (!IsObject(this) || !IsGeneratorObject(this))
            return callFunction(CallGeneratorMethodIfWrapped, this, val, "GeneratorThrow");
        if (GeneratorObjectIsClosed(this))
            throw val;
        if (GeneratorIsRunning(this))
            ThrowTypeError(38);
    }
    try {
        return resumeGenerator(this, val, "throw");
    } catch (e) {
        if (!GeneratorObjectIsClosed(this))
            GeneratorSetClosed(this);
        throw e;
    }
}
function GeneratorReturn(val) {
    if (!IsSuspendedGenerator(this)) {
        if (!IsObject(this) || !IsGeneratorObject(this))
            return callFunction(CallGeneratorMethodIfWrapped, this, val, "GeneratorReturn");
        if (GeneratorObjectIsClosed(this))
            return { value: val, done: true };
        if (GeneratorIsRunning(this))
            ThrowTypeError(38);
    }
    try {
        var rval = { value: val, done: true };
        return resumeGenerator(this, rval, "return");
    } catch (e) {
        if (!GeneratorObjectIsClosed(this))
            GeneratorSetClosed(this);
        throw e;
    }
}
function InterpretGeneratorResume(gen, val, kind) {
    forceInterpreter();
    if (kind === "next")
       return resumeGenerator(gen, val, "next");
    if (kind === "throw")
       return resumeGenerator(gen, val, "throw");
    ;;
    return resumeGenerator(gen, val, "return");
}
function IteratorIdentity() {
  return this;
}
function IteratorNext(iteratorRecord, value) {
  const result = (arguments.length < 2
      ? callContentFunction(iteratorRecord.nextMethod, iteratorRecord.iterator)
      : callContentFunction(iteratorRecord.nextMethod, iteratorRecord.iterator, value));
  if (!IsObject(result)) {
    ThrowTypeError(48, result);
  }
  return result;
}
function IteratorClose(iteratorRecord, value) {
  const iterator = iteratorRecord.iterator;
  const returnMethod = iterator.return;
  if (returnMethod !== undefined && returnMethod !== null) {
    const result = callContentFunction(returnMethod, iterator);
    if (!IsObject(result)) {
      ThrowTypeError(48, DecompileArg(0, result));
    }
  }
  return value;
}
function GetIteratorDirect(obj) {
  if (!IsObject(obj)) {
    ThrowTypeError(48, DecompileArg(0, obj));
  }
  const nextMethod = obj.next;
  if (!IsCallable(nextMethod)) {
    ThrowTypeError(10, DecompileArg(0, nextMethod));
  }
  return {
    iterator: obj,
    nextMethod,
    done: false,
  };
}
function GetIteratorDirectWrapper(obj) {
  if (!IsObject(obj)) {
    ThrowTypeError(48, obj);
  }
  const nextMethod = obj.next;
  if (!IsCallable(nextMethod)) {
    ThrowTypeError(10, nextMethod);
  }
  return {
    [std_iterator]: function IteratorMethod() {
      return this;
    },
    next(value) {
      return callContentFunction(nextMethod, obj, value);
    },
    return(value) {
      const returnMethod = obj.return;
      if (returnMethod !== undefined && returnMethod !== null) {
        return callContentFunction(returnMethod, obj, value);
      }
      return {done: true, value};
    },
  };
}
function IteratorStep(iteratorRecord, value) {
  let result;
  if (arguments.length === 2) {
    result = callContentFunction(
      iteratorRecord.nextMethod,
      iteratorRecord.iterator,
      value
    );
  } else {
    result = callContentFunction(
      iteratorRecord.nextMethod,
      iteratorRecord.iterator
    );
  }
  if (!IsObject(result)) {
    ThrowTypeError(48, DecompileArg(0, result));
  }
  return result.done ? false : result;
}
function IteratorFrom(O) {
  const usingIterator = O[std_iterator];
  let iteratorRecord;
  if (usingIterator !== undefined && usingIterator !== null) {
    const iterator = callContentFunction(usingIterator, O);
    iteratorRecord = GetIteratorDirect(iterator);
    if (iteratorRecord.iterator instanceof GetBuiltinConstructor("Iterator")) {
      return iteratorRecord.iterator;
    }
  } else {
    iteratorRecord = GetIteratorDirect(O);
  }
  const wrapper = NewWrapForValidIterator();
  UnsafeSetReservedSlot(wrapper, 0, iteratorRecord);
  return wrapper;
}
function WrapForValidIteratorNext(value) {
  let O;
  if (!IsObject(this) || (O = GuardToWrapForValidIterator(this)) === null) {
    if (arguments.length === 0) {
      return callFunction(CallWrapForValidIteratorMethodIfWrapped, this,
                          "WrapForValidIteratorNext");
    }
    return callFunction(CallWrapForValidIteratorMethodIfWrapped, this,
                        value, "WrapForValidIteratorNext");
  }
  const iterated = UnsafeGetReservedSlot(O, 0);
  let result;
  if (arguments.length === 0) {
    result = callContentFunction(iterated.nextMethod, iterated.iterator);
  } else {
    result = callContentFunction(iterated.nextMethod, iterated.iterator, value);
  }
  if (!IsObject(result))
    ThrowTypeError(48, DecompileArg(0, result));
  return result;
}
function WrapForValidIteratorReturn(value) {
  let O;
  if (!IsObject(this) || (O = GuardToWrapForValidIterator(this)) === null) {
    return callFunction(CallWrapForValidIteratorMethodIfWrapped, this,
                        value, "WrapForValidIteratorReturn");
  }
  const iterated = UnsafeGetReservedSlot(O, 0);
  const iterator = iterated.iterator;
  const returnMethod = iterator.return;
  if (returnMethod !== undefined && returnMethod !== null) {
    let innerResult = callContentFunction(returnMethod, iterator);
    if (!IsObject(innerResult)) {
      ThrowTypeError(48, DecompileArg(0, innerResult));
    }
  }
  return {
    done: true,
    value,
  };
}
function WrapForValidIteratorThrow(value) {
  let O;
  if (!IsObject(this) || (O = GuardToWrapForValidIterator(this)) === null) {
    return callFunction(CallWrapForValidIteratorMethodIfWrapped, this,
                        value, "WrapForValidIteratorThrow");
  }
  const iterated = UnsafeGetReservedSlot(O, 0);
  const iterator = iterated.iterator;
  const throwMethod = iterator.throw;
  if (throwMethod === undefined || throwMethod === null) {
    throw value;
  }
  return callContentFunction(throwMethod, iterator, value);
}
function IteratorHelperNext(value) {
  let O;
  if (!IsObject(this) || (O = GuardToIteratorHelper(this)) === null) {
    return callFunction(CallIteratorHelperMethodIfWrapped, this,
                        value, "IteratorHelperNext");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callContentFunction(GeneratorNext, generator, value);
}
function IteratorHelperReturn(value) {
  let O;
  if (!IsObject(this) || (O = GuardToIteratorHelper(this)) === null) {
    return callFunction(CallIteratorHelperMethodIfWrapped, this,
                        value, "IteratorHelperReturn");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callContentFunction(GeneratorReturn, generator, value);
}
function IteratorHelperThrow(value) {
  let O;
  if (!IsObject(this) || (O = GuardToIteratorHelper(this)) === null) {
    return callFunction(CallIteratorHelperMethodIfWrapped, this,
                        value, "IteratorHelperThrow");
  }
  const generator = UnsafeGetReservedSlot(O, 0);
  return callContentFunction(GeneratorThrow, generator, value);
}
function IteratorMap(mapper) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(mapper)) {
    ThrowTypeError(10, DecompileArg(0, mapper));
  }
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorMapGenerator(iterated, mapper);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorMapGenerator(iterated, mapper) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = IteratorStep(iterated, lastValue);
        next;
        next = IteratorStep(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      lastValue = yield callContentFunction(mapper, undefined, value);
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
}
function IteratorFilter(filterer) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(filterer)) {
    ThrowTypeError(10, DecompileArg(0, filterer));
  }
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorFilterGenerator(iterated, filterer);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorFilterGenerator(iterated, filterer) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = IteratorStep(iterated, lastValue);
        next;
        next = IteratorStep(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      if (callContentFunction(filterer, undefined, value)) {
        lastValue = yield value;
      }
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
}
function IteratorTake(limit) {
  const iterated = GetIteratorDirect(this);
  const remaining = ToInteger(limit);
  if (remaining < 0) {
    ThrowRangeError(641);
  }
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorTakeGenerator(iterated, remaining);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorTakeGenerator(iterated, remaining) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (; remaining > 0; remaining--) {
      const next = IteratorStep(iterated, lastValue);
      if (!next) {
        return;
      }
      const value = next.value;
      needClose = true;
      lastValue = yield value;
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
  IteratorClose(iterated);
}
function IteratorDrop(limit) {
  const iterated = GetIteratorDirect(this);
  const remaining = ToInteger(limit);
  if (remaining < 0) {
    ThrowRangeError(641);
  }
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorDropGenerator(iterated, remaining);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorDropGenerator(iterated, remaining) {
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (; remaining > 0; remaining--) {
      if (!IteratorStep(iterated)) {
        return;
      }
    }
    let lastValue;
    for (let next = IteratorStep(iterated, lastValue);
        next;
        next = IteratorStep(iterated, lastValue)) {
      const value = next.value;
      needClose = true;
      lastValue = yield value;
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
}
function IteratorAsIndexedPairs() {
  const iterated = GetIteratorDirect(this);
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorAsIndexedPairsGenerator(iterated);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorAsIndexedPairsGenerator(iterated) {
  let lastValue;
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = IteratorStep(iterated, lastValue), index = 0;
        next;
        next = IteratorStep(iterated, lastValue), index++) {
      const value = next.value;
      needClose = true;
      lastValue = yield [index, value];
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
}
function IteratorFlatMap(mapper) {
  const iterated = GetIteratorDirect(this);
  if (!IsCallable(mapper)) {
    ThrowTypeError(10, DecompileArg(0, mapper));
  }
  const iteratorHelper = NewIteratorHelper();
  const generator = IteratorFlatMapGenerator(iterated, mapper);
  callContentFunction(GeneratorNext, generator);
  UnsafeSetReservedSlot(iteratorHelper, 0, generator);
  return iteratorHelper;
}
function* IteratorFlatMapGenerator(iterated, mapper) {
  let needClose = true;
  try {
    yield;
    needClose = false;
    for (let next = IteratorStep(iterated);
        next;
        next = IteratorStep(iterated)) {
      const value = next.value;
      needClose = true;
      const mapped = callContentFunction(mapper, undefined, value);
      for (const innerValue of allowContentIter(mapped)) {
        yield innerValue;
      }
      needClose = false;
    }
  } finally {
    if (needClose) {
      IteratorClose(iterated);
    }
  }
}
function IteratorReduce(reducer ) {
  const iterated = GetIteratorDirectWrapper(this);
  if (!IsCallable(reducer)) {
    ThrowTypeError(10, DecompileArg(0, reducer));
  }
  let accumulator;
  if (arguments.length === 1) {
    const next = callContentFunction(iterated.next, iterated);
    if (!IsObject(next)) {
      ThrowTypeError(48, DecompileArg(0, next));
    }
    if (next.done) {
      ThrowTypeError(45);
    }
    accumulator = next.value;
  } else {
    accumulator = arguments[1];
  }
  for (const value of allowContentIter(iterated)) {
    accumulator = callContentFunction(reducer, undefined, accumulator, value);
  }
  return accumulator;
}
function IteratorToArray() {
  const iterated = {[std_iterator]: () => this};
  return [...allowContentIter(iterated)];
}
function IteratorForEach(fn) {
  const iterated = GetIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for (const value of allowContentIter(iterated)) {
    callContentFunction(fn, undefined, value);
  }
}
function IteratorSome(fn) {
  const iterated = GetIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for (const value of allowContentIter(iterated)) {
    if (callContentFunction(fn, undefined, value)) {
      return true;
    }
  }
  return false;
}
function IteratorEvery(fn) {
  const iterated = GetIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for (const value of allowContentIter(iterated)) {
    if (!callContentFunction(fn, undefined, value)) {
      return false;
    }
  }
  return true;
}
function IteratorFind(fn) {
  const iterated = GetIteratorDirectWrapper(this);
  if (!IsCallable(fn)) {
    ThrowTypeError(10, DecompileArg(0, fn));
  }
  for (const value of allowContentIter(iterated)) {
    if (callContentFunction(fn, undefined, value)) {
      return value;
    }
  }
}
function MapConstructorInit(iterable) {
    var map = this;
    var adder = map.set;
    if (!IsCallable(adder))
        ThrowTypeError(10, typeof adder);
    for (var nextItem of allowContentIter(iterable)) {
        if (!IsObject(nextItem))
            ThrowTypeError(37, "Map");
        callContentFunction(adder, map, nextItem[0], nextItem[1]);
    }
}
function MapForEach(callbackfn, thisArg = undefined) {
    var M = this;
    if (!IsObject(M) || (M = GuardToMapObject(M)) === null)
        return callFunction(CallMapMethodIfWrapped, this, callbackfn, thisArg, "MapForEach");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var entries = callFunction(std_Map_entries, M);
    var mapIterationResultPair = iteratorTemp.mapIterationResultPair;
    if (!mapIterationResultPair) {
        mapIterationResultPair = iteratorTemp.mapIterationResultPair =
            _CreateMapIterationResultPair();
    }
    while (true) {
        var done = _GetNextMapEntryForIterator(entries, mapIterationResultPair);
        if (done)
            break;
        var key = mapIterationResultPair[0];
        var value = mapIterationResultPair[1];
        mapIterationResultPair[0] = null;
        mapIterationResultPair[1] = null;
        callContentFunction(callbackfn, thisArg, value, key, M);
    }
}
var iteratorTemp = { mapIterationResultPair: null };
function MapIteratorNext() {
    var O = this;
    if (!IsObject(O) || (O = GuardToMapIterator(O)) === null)
        return callFunction(CallMapIteratorMethodIfWrapped, this, "MapIteratorNext");
    var mapIterationResultPair = iteratorTemp.mapIterationResultPair;
    if (!mapIterationResultPair) {
        mapIterationResultPair = iteratorTemp.mapIterationResultPair =
            _CreateMapIterationResultPair();
    }
    var retVal = {value: undefined, done: true};
    var done = _GetNextMapEntryForIterator(O, mapIterationResultPair);
    if (!done) {
        var itemKind = UnsafeGetInt32FromReservedSlot(O, 2);
        var result;
        if (itemKind === 0) {
            result = mapIterationResultPair[0];
        } else if (itemKind === 1) {
            result = mapIterationResultPair[1];
        } else {
            ;;
            result = [mapIterationResultPair[0], mapIterationResultPair[1]];
        }
        mapIterationResultPair[0] = null;
        mapIterationResultPair[1] = null;
        retVal.value = result;
        retVal.done = false;
    }
    return retVal;
}
function $MapSpecies() {
    return this;
}
_SetCanonicalName($MapSpecies, "get [Symbol.species]");
function CallModuleResolveHook(module, specifier, expectedMinimumStatus)
{
    let requestedModule = HostResolveImportedModule(module, specifier);
    if (requestedModule.status < expectedMinimumStatus)
        ThrowInternalError(565);
    return requestedModule;
}
function ModuleGetExportedNames(exportStarSet = [])
{
    if (!IsObject(this) || !IsModule(this)) {
        return callFunction(CallModuleMethodIfWrapped, this, exportStarSet,
                            "ModuleGetExportedNames");
    }
    let module = this;
    if (callFunction(ArrayIncludes, exportStarSet, module))
        return [];
    _DefineDataProperty(exportStarSet, exportStarSet.length, module);
    let exportedNames = [];
    let namesCount = 0;
    let localExportEntries = module.localExportEntries;
    for (let i = 0; i < localExportEntries.length; i++) {
        let e = localExportEntries[i];
        _DefineDataProperty(exportedNames, namesCount++, e.exportName);
    }
    let indirectExportEntries = module.indirectExportEntries;
    for (let i = 0; i < indirectExportEntries.length; i++) {
        let e = indirectExportEntries[i];
        _DefineDataProperty(exportedNames, namesCount++, e.exportName);
    }
    let starExportEntries = module.starExportEntries;
    for (let i = 0; i < starExportEntries.length; i++) {
        let e = starExportEntries[i];
        let requestedModule = CallModuleResolveHook(module, e.moduleRequest,
                                                    0);
        let starNames = callFunction(requestedModule.getExportedNames, requestedModule,
                                     exportStarSet);
        for (let j = 0; j < starNames.length; j++) {
            let n = starNames[j];
            if (n !== "default" && !callFunction(ArrayIncludes, exportedNames, n))
                _DefineDataProperty(exportedNames, namesCount++, n);
        }
    }
    return exportedNames;
}
function ModuleSetStatus(module, newStatus)
{
    ;;
    ;;
    UnsafeSetReservedSlot(module, 3, newStatus);
}
function ModuleResolveExport(exportName, resolveSet = [])
{
    ;;
    if (!IsObject(this) || !IsModule(this)) {
        return callFunction(CallModuleMethodIfWrapped, this, exportName, resolveSet,
                            "ModuleResolveExport");
    }
    let module = this;
    for (let i = 0; i < resolveSet.length; i++) {
        let r = resolveSet[i];
        if (r.module === module && r.exportName === exportName) {
            return null;
        }
    }
    _DefineDataProperty(resolveSet, resolveSet.length, {module, exportName});
    let localExportEntries = module.localExportEntries;
    for (let i = 0; i < localExportEntries.length; i++) {
        let e = localExportEntries[i];
        if (exportName === e.exportName)
            return {module, bindingName: e.localName};
    }
    let indirectExportEntries = module.indirectExportEntries;
    for (let i = 0; i < indirectExportEntries.length; i++) {
        let e = indirectExportEntries[i];
        if (exportName === e.exportName) {
            let importedModule = CallModuleResolveHook(module, e.moduleRequest,
                                                       0);
            if (e.importName === null) {
                return {module: importedModule, bindingName: "*namespace*"};
            }
            return callFunction(importedModule.resolveExport, importedModule, e.importName,
                                resolveSet);
        }
    }
    if (exportName === "default") {
        return null;
    }
    let starResolution = null;
    let starExportEntries = module.starExportEntries;
    for (let i = 0; i < starExportEntries.length; i++) {
        let e = starExportEntries[i];
        let importedModule = CallModuleResolveHook(module, e.moduleRequest,
                                                   0);
        let resolution = callFunction(importedModule.resolveExport, importedModule, exportName,
                                      resolveSet);
        if (resolution === "ambiguous")
            return resolution;
        if (resolution !== null) {
            if (starResolution === null) {
                starResolution = resolution;
            } else {
                if (resolution.module !== starResolution.module ||
                    resolution.bindingName !== starResolution.bindingName)
                {
                    return "ambiguous";
                }
            }
        }
    }
    return starResolution;
}
function IsResolvedBinding(resolution)
{
    ;;
    return typeof resolution === "object" && resolution !== null;
}
function GetModuleNamespace(module)
{
    ;;
    ;;
    let namespace = module.namespace;
    if (typeof namespace === "undefined") {
        let exportedNames = callFunction(module.getExportedNames, module);
        let unambiguousNames = [];
        for (let i = 0; i < exportedNames.length; i++) {
            let name = exportedNames[i];
            let resolution = callFunction(module.resolveExport, module, name);
            if (IsResolvedBinding(resolution))
                _DefineDataProperty(unambiguousNames, unambiguousNames.length, name);
        }
        namespace = ModuleNamespaceCreate(module, unambiguousNames);
    }
    return namespace;
}
function ModuleNamespaceCreate(module, exports)
{
    callFunction(ArraySort, exports);
    let ns = NewModuleNamespace(module, exports);
    for (let i = 0; i < exports.length; i++) {
        let name = exports[i];
        let binding = callFunction(module.resolveExport, module, name);
        ;;
        if (binding.bindingName === "*namespace*") {
            let namespace = GetModuleNamespace(binding.module);
            EnsureModuleEnvironmentNamespace(binding.module, namespace);
            AddModuleNamespaceBinding(ns, name, binding.module, binding.bindingName);
        } else {
            AddModuleNamespaceBinding(ns, name, binding.module, binding.bindingName);
        }
    }
    return ns;
}
function GetModuleEnvironment(module)
{
    ;;
    ;;
    let env = UnsafeGetReservedSlot(module, 1);
    ;;
    return env;
}
function CountArrayValues(array, value)
{
    let count = 0;
    for (let i = 0; i < array.length; i++) {
        if (array[i] === value)
            count++;
    }
    return count;
}
function ArrayContains(array, value)
{
    for (let i = 0; i < array.length; i++) {
        if (array[i] === value)
            return true;
    }
    return false;
}
function HandleModuleInstantiationFailure(module)
{
    ModuleSetStatus(module, 0);
    UnsafeSetReservedSlot(module, 14, undefined);
    UnsafeSetReservedSlot(module, 15, undefined);
}
function ModuleInstantiate()
{
    if (!IsObject(this) || !IsModule(this))
        return callFunction(CallModuleMethodIfWrapped, this, "ModuleInstantiate");
    let module = this;
    if (module.status === 1 ||
        module.status === 3)
    {
        ThrowInternalError(565);
    }
    let stack = [];
    try {
        InnerModuleLinking(module, stack, 0);
    } catch (error) {
        for (let i = 0; i < stack.length; i++) {
            let m = stack[i];
            if (m.status === 1) {
                HandleModuleInstantiationFailure(m);
            }
        }
        if (stack.length === 0 && module.status === 1) {
            HandleModuleInstantiationFailure(module);
        }
        ;;
        throw error;
    }
    ;;
    ;;
    return undefined;
}
function InnerModuleLinking(module, stack, index)
{
    if (module.status === 1 ||
        module.status === 2 ||
        module.status === 4 ||
        module.status === 5)
    {
        return index;
    }
    if (module.status !== 0)
        ThrowInternalError(565);
    ModuleSetStatus(module, 1);
    UnsafeSetReservedSlot(module, 14, index);
    UnsafeSetReservedSlot(module, 15, index);
    index++;
    _DefineDataProperty(stack, stack.length, module);
    let requestedModules = module.requestedModules;
    for (let i = 0; i < requestedModules.length; i++) {
        let required = requestedModules[i].moduleSpecifier;
        let requiredModule = CallModuleResolveHook(module, required, 0);
        index = InnerModuleLinking(requiredModule, stack, index);
        ;;
        ;;
        ;;
        ;;
        if (requiredModule.status === 1) {
            UnsafeSetReservedSlot(module, 15,
                                  std_Math_min(module.dfsAncestorIndex,
                                               requiredModule.dfsAncestorIndex));
        }
    }
    callFunction(InitializeEnvironment, module);
    ;;
    ;;
    if (module.dfsAncestorIndex === module.dfsIndex) {
        let requiredModule;
        do {
            requiredModule = callFunction(std_Array_pop, stack);
            ModuleSetStatus(requiredModule, 2);
        } while (requiredModule !== module);
    }
    return index;
}
function InitializeEnvironment()
{
    let module = this;
    let indirectExportEntries = module.indirectExportEntries;
    for (let i = 0; i < indirectExportEntries.length; i++) {
        let e = indirectExportEntries[i];
        let resolution = callFunction(module.resolveExport, module, e.exportName);
        if (!IsResolvedBinding(resolution)) {
            ThrowResolutionError(module, resolution, "indirectExport", e.exportName,
                                 e.lineNumber, e.columnNumber);
        }
    }
    let env = GetModuleEnvironment(module);
    let importEntries = module.importEntries;
    for (let i = 0; i < importEntries.length; i++) {
        let imp = importEntries[i];
        let importedModule = CallModuleResolveHook(module, imp.moduleRequest,
                                                   1);
        if (imp.importName === null) {
            let namespace = GetModuleNamespace(importedModule);
            CreateNamespaceBinding(env, imp.localName, namespace);
        } else {
            let resolution = callFunction(importedModule.resolveExport, importedModule,
                                          imp.importName);
            if (!IsResolvedBinding(resolution)) {
                ThrowResolutionError(module, resolution, "import", imp.importName,
                                     imp.lineNumber, imp.columnNumber);
            }
            if (resolution.bindingName === "*namespace*") {
                let namespace = GetModuleNamespace(resolution.module);
                EnsureModuleEnvironmentNamespace(resolution.module, namespace);
                CreateImportBinding(env, imp.localName, resolution.module,
                                    resolution.bindingName);
            } else {
                CreateImportBinding(env, imp.localName, resolution.module,
                                    resolution.bindingName);
            }
        }
    }
    InstantiateModuleFunctionDeclarations(module);
}
function ThrowResolutionError(module, resolution, kind, name, line, column)
{
    ;;
    ;;
    ;;
    let ambiguous = resolution === "ambiguous";
    let errorNumber;
    if (kind === "import")
        errorNumber = ambiguous ? 562 : 561;
    else
        errorNumber = ambiguous ? 560 : 559;
    let message = GetErrorMessage(errorNumber) + ": " + name;
    let error = CreateModuleSyntaxError(module, line, column, message);
    throw error;
}
function GetModuleEvaluationError(module)
{
    ;;
    ;;
    return UnsafeGetReservedSlot(module, 4);
}
function RecordModuleEvaluationError(module, error)
{
    ;;
    if (module.status === 5) {
        return;
    }
    ModuleSetStatus(module, 5);
    UnsafeSetReservedSlot(module, 4, error);
}
function ModuleEvaluate()
{
    if (!IsObject(this) || !IsModule(this))
        return callFunction(CallModuleMethodIfWrapped, this, "ModuleEvaluate");
    const isTopLevelAwaitEnabled = IsTopLevelAwaitEnabled();
    let module = this;
    if (module.status !== 2 &&
        module.status !== 4 &&
        module.status !== 5)
    {
        ThrowInternalError(565);
    }
    let capability = undefined;
    if (isTopLevelAwaitEnabled) {
      if (module.status === 4) {
        module = GetAsyncCycleRoot(module);
      }
      if (module.topLevelCapability) {
        return module.topLevelCapability;
      }
      capability = CreateTopLevelCapability(module);
    }
    let stack = [];
    try {
        InnerModuleEvaluation(module, stack, 0);
        if (isTopLevelAwaitEnabled) {
          if (!module.asyncEvaluating) {
            ModuleTopLevelCapabilityResolve(module);
          }
          ;;
          ;;
        }
    } catch (error) {
        for (let i = 0; i < stack.length; i++) {
            let m = stack[i];
            ;;
            RecordModuleEvaluationError(m, error);
        }
        if (stack.length === 0)
            RecordModuleEvaluationError(module, error);
        ;;
        if (isTopLevelAwaitEnabled) {
          ModuleTopLevelCapabilityReject(module, error);
        } else {
          throw error;
        }
    }
    return capability;
}
function InnerModuleEvaluation(module, stack, index)
{
    const isTopLevelAwaitEnabled = IsTopLevelAwaitEnabled();
    if (module.status === 5)
        throw GetModuleEvaluationError(module);
    if (module.status === 4)
        return index;
    if (module.status === 3)
        return index;
    ;;
    ModuleSetStatus(module, 3);
    UnsafeSetReservedSlot(module, 14, index);
    UnsafeSetReservedSlot(module, 15, index);
    if (isTopLevelAwaitEnabled) {
      UnsafeSetReservedSlot(module, 20, 0);
    }
    index++;
    _DefineDataProperty(stack, stack.length, module);
    let requestedModules = module.requestedModules;
    for (let i = 0; i < requestedModules.length; i++) {
        let required = requestedModules[i].moduleSpecifier;
        let requiredModule =
            CallModuleResolveHook(module, required, 2);
        index = InnerModuleEvaluation(requiredModule, stack, index);
        ;;
        ;;
        ;;
        ;;
        if (requiredModule.status === 3) {
            UnsafeSetReservedSlot(module, 15,
                                  std_Math_min(module.dfsAncestorIndex,
                                               requiredModule.dfsAncestorIndex));
        } else {
          if (isTopLevelAwaitEnabled) {
            requiredModule = GetAsyncCycleRoot(requiredModule);
            ;;
            if (requiredModule.evaluationError) {
              throw GetModuleEvaluationError(module);
            }
          }
        }
        if (isTopLevelAwaitEnabled) {
          if (requiredModule.asyncEvaluating) {
              UnsafeSetReservedSlot(module,
                                    20,
                                    module.pendingAsyncDependencies + 1);
              AppendAsyncParentModule(requiredModule, module);
          }
        }
    }
    if (isTopLevelAwaitEnabled) {
      if (module.pendingAsyncDependencies > 0) {
        UnsafeSetReservedSlot(module, 17, true);
      } else {
        if (module.async) {
          ExecuteAsyncModule(module);
        } else {
          ExecuteModule(module);
        }
      }
    } else {
      ExecuteModule(module);
    }
    ;;
    ;;
    if (module.dfsAncestorIndex === module.dfsIndex) {
        let requiredModule;
        do {
            requiredModule = callFunction(std_Array_pop, stack);
            ModuleSetStatus(requiredModule, 4);
        } while (requiredModule !== module);
    }
    return index;
}
function ExecuteAsyncModule(module) {
  ;;
  ;;
  UnsafeSetReservedSlot(module, 17, true);
  ExecuteModule(module);
}
var numberFormatCache = new Record();
function Number_toLocaleString() {
    var x = callFunction(ThisNumberValueForToLocaleString, this);
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var numberFormat;
    if (locales === undefined && options === undefined) {
        if (!IsRuntimeDefaultLocale(numberFormatCache.runtimeDefaultLocale)) {
            numberFormatCache.numberFormat = intl_NumberFormat(locales, options);
            numberFormatCache.runtimeDefaultLocale = RuntimeDefaultLocale();
        }
        numberFormat = numberFormatCache.numberFormat;
    } else {
        numberFormat = intl_NumberFormat(locales, options);
    }
    return intl_FormatNumber(numberFormat, x, false, false);
}
function Number_isFinite(num) {
    if (typeof num !== "number")
        return false;
    return num - num === 0;
}
function Number_isNaN(num) {
    if (typeof num !== "number")
        return false;
    return num !== num;
}
function Number_isInteger(number) {
    if (typeof number !== "number")
        return false;
    var integer = std_Math_trunc(number);
    return number - integer === 0;
}
function Number_isSafeInteger(number) {
    if (typeof number !== "number")
        return false;
    var integer = std_Math_trunc(number);
    if (number - integer !== 0)
        return false;
    return -((2 ** 53) - 1) <= integer && integer <= (2 ** 53) - 1;
}
function Global_isNaN(number) {
    return Number_isNaN(ToNumber(number));
}
function Global_isFinite(number) {
    return Number_isFinite(ToNumber(number));
}
function ObjectGetOwnPropertyDescriptors(O) {
    var obj = ToObject(O);
    var keys = std_Reflect_ownKeys(obj);
    var descriptors = {};
    for (var index = 0, len = keys.length; index < len; index++) {
        var key = keys[index];
        var desc = ObjectGetOwnPropertyDescriptor(obj, key);
        if (typeof desc !== "undefined")
            _DefineDataProperty(descriptors, key, desc);
    }
    return descriptors;
}
function ObjectGetPrototypeOf(obj) {
    return std_Reflect_getPrototypeOf(ToObject(obj));
}
function ObjectIsExtensible(obj) {
    return IsObject(obj) && std_Reflect_isExtensible(obj);
}
function Object_toLocaleString() {
    var O = this;
    return callContentFunction(O.toString, O);
}
function Object_valueOf() {
    return ToObject(this);
}
function Object_hasOwnProperty(V) {
    return hasOwn(V, this);
}
function $ObjectProtoGetter() {
    return std_Reflect_getPrototypeOf(ToObject(this));
}
_SetCanonicalName($ObjectProtoGetter, "get __proto__");
function $ObjectProtoSetter(proto) {
    return callFunction(std_Object_setProto, this, proto);
}
_SetCanonicalName($ObjectProtoSetter, "set __proto__");
function ObjectDefineSetter(name, setter) {
    var object = ToObject(this);
    if (!IsCallable(setter))
        ThrowTypeError(26, "setter");
    var key = (typeof name !== "string" && typeof name !== "number" && typeof name !== "symbol" ? ToPropertyKey(name) : name);
    _DefineProperty(object, key, 0x200 | 0x01 | 0x02,
                    null, setter, true);
}
function ObjectDefineGetter(name, getter) {
    var object = ToObject(this);
    if (!IsCallable(getter))
        ThrowTypeError(26, "getter");
    var key = (typeof name !== "string" && typeof name !== "number" && typeof name !== "symbol" ? ToPropertyKey(name) : name);
    _DefineProperty(object, key, 0x200 | 0x01 | 0x02,
                    getter, null, true);
}
function ObjectLookupSetter(name) {
    var object = ToObject(this);
    var key = (typeof name !== "string" && typeof name !== "number" && typeof name !== "symbol" ? ToPropertyKey(name) : name);
    do {
        var desc = GetOwnPropertyDescriptorToArray(object, key);
        if (desc) {
            if (desc[0] & 0x200)
                return desc[2];
            return undefined;
        }
        object = std_Reflect_getPrototypeOf(object);
    } while (object !== null);
}
function ObjectLookupGetter(name) {
    var object = ToObject(this);
    var key = (typeof name !== "string" && typeof name !== "number" && typeof name !== "symbol" ? ToPropertyKey(name) : name);
    do {
        var desc = GetOwnPropertyDescriptorToArray(object, key);
        if (desc) {
            if (desc[0] & 0x200)
                return desc[1];
            return undefined;
        }
        object = std_Reflect_getPrototypeOf(object);
    } while (object !== null);
}
function ObjectGetOwnPropertyDescriptor(obj, propertyKey) {
    var desc = GetOwnPropertyDescriptorToArray(obj, propertyKey);
    if (!desc)
        return undefined;
    var attrsAndKind = desc[0];
    if (attrsAndKind & 0x100) {
        return {
            value: desc[1],
            writable: !!(attrsAndKind & 0x04),
            enumerable: !!(attrsAndKind & 0x01),
            configurable: !!(attrsAndKind & 0x02),
        };
    }
    ;;
    return {
        get: desc[1],
        set: desc[2],
        enumerable: !!(attrsAndKind & 0x01),
        configurable: !!(attrsAndKind & 0x02),
    };
}
function ObjectOrReflectDefineProperty(obj, propertyKey, attributes, strict) {
    if (!IsObject(obj))
        ThrowTypeError(48, DecompileArg(0, obj));
    propertyKey = (typeof propertyKey !== "string" && typeof propertyKey !== "number" && typeof propertyKey !== "symbol" ? ToPropertyKey(propertyKey) : propertyKey);
    if (!IsObject(attributes))
        ThrowArgTypeNotObject(0, attributes);
    var attrs = 0, hasValue = false;
    var value, getter = null, setter = null;
    if ("enumerable" in attributes)
        attrs |= attributes.enumerable ? 0x01 : 0x08;
    if ("configurable" in attributes)
        attrs |= attributes.configurable ? 0x02 : 0x10;
    if ("value" in attributes) {
        attrs |= 0x100;
        value = attributes.value;
        hasValue = true;
    }
    if ("writable" in attributes) {
        attrs |= 0x100;
        attrs |= attributes.writable ? 0x04 : 0x20;
    }
    if ("get" in attributes) {
        attrs |= 0x200;
        getter = attributes.get;
        if (!IsCallable(getter) && getter !== undefined)
            ThrowTypeError(62, "get");
    }
    if ("set" in attributes) {
        attrs |= 0x200;
        setter = attributes.set;
        if (!IsCallable(setter) && setter !== undefined)
            ThrowTypeError(62, "set");
    }
    if (attrs & 0x200) {
        if (attrs & 0x100)
            ThrowTypeError(56);
        return _DefineProperty(obj, propertyKey, attrs, getter, setter, strict);
    }
    if (hasValue) {
        if (strict) {
            if ((attrs & (0x01 | 0x02 | 0x04)) ===
                (0x01 | 0x02 | 0x04))
            {
                _DefineDataProperty(obj, propertyKey, value);
                return true;
            }
        }
        return _DefineProperty(obj, propertyKey, attrs, value, null, strict);
    }
    return _DefineProperty(obj, propertyKey, attrs, undefined, undefined, strict);
}
function ObjectDefineProperty(obj, propertyKey, attributes) {
    if (!ObjectOrReflectDefineProperty(obj, propertyKey, attributes, true)) {
        return null;
    }
    return obj;
}
function ObjectFromEntries(iter) {
    const obj = {};
    for (const pair of allowContentIter(iter)) {
        if (!IsObject(pair))
            ThrowTypeError(37, "Object.fromEntries");
        _DefineDataProperty(obj, pair[0], pair[1]);
    }
    return obj;
}
function Promise_finally(onFinally) {
    var promise = this;
    if (!IsObject(promise))
        ThrowTypeError(3, "Promise", "finally", "value");
    var C = SpeciesConstructor(promise, GetBuiltinConstructor("Promise"));
    ;;
    var thenFinally, catchFinally;
    if (!IsCallable(onFinally)) {
        thenFinally = onFinally;
        catchFinally = onFinally;
    } else {
        (thenFinally) = function(value) {
            var result = onFinally();
            var promise = PromiseResolve(C, result);
            return callContentFunction(promise.then, promise, function() { return value; });
        };
        (catchFinally) = function(reason) {
            var result = onFinally();
            var promise = PromiseResolve(C, result);
            return callContentFunction(promise.then, promise, function() { throw reason; });
        };
    }
    return callContentFunction(promise.then, promise, thenFinally, catchFinally);
}
function CreateListFromArrayLikeForArgs(obj) {
    ;;
    var len = ToLength(obj.length);
    if (len > (500 * 1000))
        ThrowRangeError(110);
    var list = std_Array(len);
    for (var i = 0; i < len; i++)
        _DefineDataProperty(list, i, obj[i]);
    return list;
}
function Reflect_apply(target, thisArgument, argumentsList) {
    if (!IsCallable(target))
        ThrowTypeError(10, DecompileArg(0, target));
    if (!IsObject(argumentsList)) {
        ThrowTypeError(49, "`argumentsList`", "Reflect.apply",
                       ToSource(argumentsList));
    }
    return callFunction(std_Function_apply, target, thisArgument, argumentsList);
}
function Reflect_construct(target, argumentsList ) {
    if (!IsConstructor(target))
        ThrowTypeError(11, DecompileArg(0, target));
    var newTarget;
    if (arguments.length > 2) {
        newTarget = arguments[2];
        if (!IsConstructor(newTarget))
            ThrowTypeError(11, DecompileArg(2, newTarget));
    } else {
        newTarget = target;
    }
    if (!IsObject(argumentsList)) {
        ThrowTypeError(49, "`argumentsList`", "Reflect.construct",
                       ToSource(argumentsList));
    }
    var args = (IsPackedArray(argumentsList) && argumentsList.length <= (500 * 1000))
               ? argumentsList
               : CreateListFromArrayLikeForArgs(argumentsList);
    switch (args.length) {
      case 0:
        return constructContentFunction(target, newTarget);
      case 1:
        return constructContentFunction(target, newTarget, args[0]);
      case 2:
        return constructContentFunction(target, newTarget, args[0], args[1]);
      case 3:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2]);
      case 4:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3]);
      case 5:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4]);
      case 6:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5]);
      case 7:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
      case 8:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
      case 9:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
      case 10:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
      case 11:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]);
      case 12:
        return constructContentFunction(target, newTarget, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]);
      default:
        return _ConstructFunction(target, newTarget, args);
    }
}
function Reflect_defineProperty(obj, propertyKey, attributes) {
    return ObjectOrReflectDefineProperty(obj, propertyKey, attributes, false);
}
function Reflect_getOwnPropertyDescriptor(target, propertyKey) {
    if (!IsObject(target))
        ThrowTypeError(48, DecompileArg(0, target));
    return ObjectGetOwnPropertyDescriptor(target, propertyKey);
}
function Reflect_has(target, propertyKey) {
    if (!IsObject(target)) {
        ThrowTypeError(49, "`target`", "Reflect.has",
                       ToSource(target));
    }
    return propertyKey in target;
}
function Reflect_get(target, propertyKey ) {
    if (!IsObject(target)) {
        ThrowTypeError(49, "`target`", "Reflect.get",
                       ToSource(target));
    }
    if (arguments.length > 2) {
        return getPropertySuper(target, propertyKey, arguments[2]);
    }
    return target[propertyKey];
}
function $RegExpFlagsGetter() {
    var R = this;
    if (!IsObject(R))
        ThrowTypeError(48, R === null ? "null" : typeof R);
    var result = "";
    if (R.global)
        result += "g";
    if (R.ignoreCase)
        result += "i";
    if (R.multiline)
        result += "m";
    if (R.dotAll)
        result += "s";
    if (R.unicode)
         result += "u";
    if (R.sticky)
        result += "y";
    return result;
}
_SetCanonicalName($RegExpFlagsGetter, "get flags");
function $RegExpToString()
{
    var R = this;
    if (!IsObject(R))
        ThrowTypeError(48, R === null ? "null" : typeof R);
    var pattern = ToString(R.source);
    var flags = ToString(R.flags);
    return "/" + pattern + "/" + flags;
}
_SetCanonicalName($RegExpToString, "toString");
function AdvanceStringIndex(S, index) {
    ;;
    ;;
    var length = S.length;
    if (index + 1 >= length)
        return index + 1;
    var first = callFunction(std_String_charCodeAt, S, index);
    if (first < 0xD800 || first > 0xDBFF)
        return index + 1;
    var second = callFunction(std_String_charCodeAt, S, index + 1);
    if (second < 0xDC00 || second > 0xDFFF)
        return index + 1;
    return index + 2;
}
function RegExpMatch(string) {
    var rx = this;
    if (!IsObject(rx))
        ThrowTypeError(48, rx === null ? "null" : typeof rx);
    var S = ToString(string);
    if (IsRegExpMethodOptimizable(rx)) {
        var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
        var global = !!(flags & 0x02);
        if (global) {
            var fullUnicode = !!(flags & 0x10);
            return RegExpGlobalMatchOpt(rx, S, fullUnicode);
        }
        return RegExpBuiltinExec(rx, S, false);
    }
    return RegExpMatchSlowPath(rx, S);
}
function RegExpMatchSlowPath(rx, S) {
    if (!rx.global)
        return RegExpExec(rx, S, false);
    var fullUnicode = !!rx.unicode;
    rx.lastIndex = 0;
    var A = [];
    var n = 0;
    while (true) {
        var result = RegExpExec(rx, S, false);
        if (result === null)
          return (n === 0) ? null : A;
        var matchStr = ToString(result[0]);
        _DefineDataProperty(A, n, matchStr);
        if (matchStr === "") {
            var lastIndex = ToLength(rx.lastIndex);
            rx.lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
        }
        n++;
    }
}
function RegExpGlobalMatchOpt(rx, S, fullUnicode) {
    var lastIndex = 0;
    rx.lastIndex = 0;
    var A = [];
    var n = 0;
    var lengthS = S.length;
    while (true) {
        var result = RegExpMatcher(rx, S, lastIndex);
        if (result === null)
            return (n === 0) ? null : A;
        lastIndex = result.index + result[0].length;
        var matchStr = result[0];
        _DefineDataProperty(A, n, matchStr);
        if (matchStr === "") {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                return A;
        }
        n++;
    }
}
function IsRegExpMethodOptimizable(rx) {
    if (!IsRegExpObject(rx))
        return false;
    var RegExpProto = GetBuiltinPrototype("RegExp");
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpInstanceOptimizable(rx, RegExpProto) &&
           RegExpProto.exec === RegExp_prototype_Exec;
}
function RegExpReplace(string, replaceValue) {
    var rx = this;
    if (!IsObject(rx))
        ThrowTypeError(48, rx === null ? "null" : typeof rx);
    var S = ToString(string);
    var lengthS = S.length;
    var functionalReplace = IsCallable(replaceValue);
    var firstDollarIndex = -1;
    if (!functionalReplace) {
        replaceValue = ToString(replaceValue);
        if (replaceValue.length > 1)
            firstDollarIndex = GetFirstDollarIndex(replaceValue);
    }
    if (IsRegExpMethodOptimizable(rx)) {
        var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
        var global = !!(flags & 0x02);
        if (global) {
            if (functionalReplace) {
                if (lengthS > 5000) {
                    var elemBase = GetElemBaseForLambda(replaceValue);
                    if (IsObject(elemBase)) {
                        return RegExpGlobalReplaceOptElemBase(rx, S, lengthS, replaceValue, flags,
                                                              elemBase);
                    }
                }
                return RegExpGlobalReplaceOptFunc(rx, S, lengthS, replaceValue, flags);
            }
            if (firstDollarIndex !== -1) {
                return RegExpGlobalReplaceOptSubst(rx, S, lengthS, replaceValue, flags,
                                                   firstDollarIndex);
            }
            if (lengthS < 0x7fff)
                return RegExpGlobalReplaceShortOpt(rx, S, lengthS, replaceValue, flags);
            return RegExpGlobalReplaceOpt(rx, S, lengthS, replaceValue, flags);
        }
        if (functionalReplace)
            return RegExpLocalReplaceOptFunc(rx, S, lengthS, replaceValue);
        if (firstDollarIndex !== -1)
            return RegExpLocalReplaceOptSubst(rx, S, lengthS, replaceValue, firstDollarIndex);
        if (lengthS < 0x7fff)
            return RegExpLocalReplaceOptShort(rx, S, lengthS, replaceValue);
        return RegExpLocalReplaceOpt(rx, S, lengthS, replaceValue);
    }
    return RegExpReplaceSlowPath(rx, S, lengthS, replaceValue,
                                 functionalReplace, firstDollarIndex);
}
function RegExpReplaceSlowPath(rx, S, lengthS, replaceValue,
                               functionalReplace, firstDollarIndex)
{
    var global = !!rx.global;
    var fullUnicode = false;
    if (global) {
        fullUnicode = !!rx.unicode;
        rx.lastIndex = 0;
    }
    var results = [];
    var nResults = 0;
    while (true) {
        var result = RegExpExec(rx, S, false);
        if (result === null)
            break;
        _DefineDataProperty(results, nResults++, result);
        if (!global)
            break;
        var matchStr = ToString(result[0]);
        if (matchStr === "") {
            var lastIndex = ToLength(rx.lastIndex);
            rx.lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
        }
    }
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    for (var i = 0; i < nResults; i++) {
        result = results[i];
        var nCaptures = std_Math_max(ToLength(result.length) - 1, 0);
        var matched = ToString(result[0]);
        var matchLength = matched.length;
        var position = std_Math_max(std_Math_min(ToInteger(result.index), lengthS), 0);
        var n, capN, replacement;
        if (functionalReplace || firstDollarIndex !== -1) {
            replacement = RegExpGetComplexReplacement(result, matched, S, position,
                                                      nCaptures, replaceValue,
                                                      functionalReplace, firstDollarIndex);
        } else {
            for (n = 1; n <= nCaptures; n++) {
                capN = result[n];
                if (capN !== undefined)
                    ToString(capN);
            }
            var namedCaptures = result.groups;
            if (namedCaptures !== undefined)
                ToObject(namedCaptures);
            replacement = replaceValue;
        }
        if (position >= nextSourcePosition) {
            accumulatedResult += Substring(S, nextSourcePosition,
                                           position - nextSourcePosition) + replacement;
            nextSourcePosition = position + matchLength;
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpGetComplexReplacement(result, matched, S, position,
                                     nCaptures, replaceValue,
                                     functionalReplace, firstDollarIndex)
{
    var captures = [];
    var capturesLength = 0;
    _DefineDataProperty(captures, capturesLength++, matched);
    for (var n = 1; n <= nCaptures; n++) {
        var capN = result[n];
        if (capN !== undefined)
            capN = ToString(capN);
        _DefineDataProperty(captures, capturesLength++, capN);
    }
    var namedCaptures = result.groups;
    if (functionalReplace) {
        if (namedCaptures === undefined) {
            switch (nCaptures) {
              case 0:
                return ToString(replaceValue(captures[0], position, S));
              case 1:
                return ToString(replaceValue(captures[0], captures[1], position, S));
              case 2:
                return ToString(replaceValue(captures[0], captures[1], captures[2], position, S));
              case 3:
                return ToString(replaceValue(captures[0], captures[1], captures[2], captures[3], position, S));
              case 4:
                return ToString(replaceValue(captures[0], captures[1], captures[2], captures[3], captures[4], position, S));
            }
        }
        _DefineDataProperty(captures, capturesLength++, position);
        _DefineDataProperty(captures, capturesLength++, S);
        if (namedCaptures !== undefined) {
            _DefineDataProperty(captures, capturesLength++, namedCaptures);
        }
        return ToString(callFunction(std_Function_apply, replaceValue, undefined, captures));
    }
    if (namedCaptures !== undefined) {
        namedCaptures = ToObject(namedCaptures);
    }
    return RegExpGetSubstitution(captures, S, position, replaceValue, firstDollarIndex,
                                 namedCaptures);
}
function RegExpGetFunctionalReplacement(result, S, position, replaceValue) {
    ;;
    var nCaptures = result.length - 1;
    var namedCaptures = result.groups;
    if (namedCaptures === undefined) {
        switch (nCaptures) {
          case 0:
            return ToString(replaceValue(result[0], position, S));
          case 1:
            return ToString(replaceValue(result[0], result[1], position, S));
          case 2:
            return ToString(replaceValue(result[0], result[1], result[2], position, S));
          case 3:
            return ToString(replaceValue(result[0], result[1], result[2], result[3], position, S));
          case 4:
            return ToString(replaceValue(result[0], result[1], result[2], result[3], result[4], position, S));
        }
    }
    var captures = [];
    for (var n = 0; n <= nCaptures; n++) {
        ;;
        _DefineDataProperty(captures, n, result[n]);
    }
    _DefineDataProperty(captures, nCaptures + 1, position);
    _DefineDataProperty(captures, nCaptures + 2, S);
    if (namedCaptures !== undefined) {
        _DefineDataProperty(captures, nCaptures + 3, namedCaptures);
    }
    return ToString(callFunction(std_Function_apply, replaceValue, undefined, captures));
}
function RegExpGlobalReplaceShortOpt(rx, S, lengthS, replaceValue, flags)
{
    var fullUnicode = !!(flags & 0x10);
    var lastIndex = 0;
    rx.lastIndex = 0;
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    while (true) {
        var result = RegExpSearcher(rx, S, lastIndex);
        if (result === -1)
            break;
        var position = result & 0x7fff;
        lastIndex = (result >> 15) & 0x7fff;
        accumulatedResult += Substring(S, nextSourcePosition,
                                       position - nextSourcePosition) + replaceValue;
        nextSourcePosition = lastIndex;
        if (lastIndex === position) {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                break;
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpGlobalReplaceOpt(rx, S, lengthS, replaceValue, flags
                  )
{
    var fullUnicode = !!(flags & 0x10);
    var lastIndex = 0;
    rx.lastIndex = 0;
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    while (true) {
        var result = RegExpMatcher(rx, S, lastIndex);
        if (result === null)
            break;
        ;;
        var matched = result[0];
        var matchLength = matched.length | 0;
        var position = result.index | 0;
        lastIndex = position + matchLength;
        var replacement;
        replacement = replaceValue;
        accumulatedResult += Substring(S, nextSourcePosition,
                                       position - nextSourcePosition) + replacement;
        nextSourcePosition = lastIndex;
        if (matchLength === 0) {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                break;
            lastIndex |= 0;
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpGlobalReplaceOptFunc(rx, S, lengthS, replaceValue, flags
                  )
{
    var fullUnicode = !!(flags & 0x10);
    var lastIndex = 0;
    rx.lastIndex = 0;
    var originalSource = UnsafeGetStringFromReservedSlot(rx, 1);
    var originalFlags = flags;
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    while (true) {
        var result = RegExpMatcher(rx, S, lastIndex);
        if (result === null)
            break;
        ;;
        var matched = result[0];
        var matchLength = matched.length | 0;
        var position = result.index | 0;
        lastIndex = position + matchLength;
        var replacement;
        replacement = RegExpGetFunctionalReplacement(result, S, position, replaceValue);
        accumulatedResult += Substring(S, nextSourcePosition,
                                       position - nextSourcePosition) + replacement;
        nextSourcePosition = lastIndex;
        if (matchLength === 0) {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                break;
            lastIndex |= 0;
        }
        if (UnsafeGetStringFromReservedSlot(rx, 1) !== originalSource ||
            UnsafeGetInt32FromReservedSlot(rx, 2) !== originalFlags)
        {
            rx = regexp_construct_raw_flags(originalSource, originalFlags);
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpGlobalReplaceOptElemBase(rx, S, lengthS, replaceValue, flags
                   , elemBase
                  )
{
    var fullUnicode = !!(flags & 0x10);
    var lastIndex = 0;
    rx.lastIndex = 0;
    var originalSource = UnsafeGetStringFromReservedSlot(rx, 1);
    var originalFlags = flags;
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    while (true) {
        var result = RegExpMatcher(rx, S, lastIndex);
        if (result === null)
            break;
        ;;
        var matched = result[0];
        var matchLength = matched.length | 0;
        var position = result.index | 0;
        lastIndex = position + matchLength;
        var replacement;
        if (IsObject(elemBase)) {
            var prop = GetStringDataProperty(elemBase, matched);
            if (prop !== undefined) {
                ;;
                replacement = prop;
            } else {
                elemBase = undefined;
            }
        }
        if (!IsObject(elemBase))
            replacement = RegExpGetFunctionalReplacement(result, S, position, replaceValue);
        accumulatedResult += Substring(S, nextSourcePosition,
                                       position - nextSourcePosition) + replacement;
        nextSourcePosition = lastIndex;
        if (matchLength === 0) {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                break;
            lastIndex |= 0;
        }
        if (UnsafeGetStringFromReservedSlot(rx, 1) !== originalSource ||
            UnsafeGetInt32FromReservedSlot(rx, 2) !== originalFlags)
        {
            rx = regexp_construct_raw_flags(originalSource, originalFlags);
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpGlobalReplaceOptSubst(rx, S, lengthS, replaceValue, flags
                   , firstDollarIndex
                  )
{
    var fullUnicode = !!(flags & 0x10);
    var lastIndex = 0;
    rx.lastIndex = 0;
    var accumulatedResult = "";
    var nextSourcePosition = 0;
    while (true) {
        var result = RegExpMatcher(rx, S, lastIndex);
        if (result === null)
            break;
        ;;
        var matched = result[0];
        var matchLength = matched.length | 0;
        var position = result.index | 0;
        lastIndex = position + matchLength;
        var replacement;
        var namedCaptures = result.groups;
        if (namedCaptures !== undefined) {
            namedCaptures = ToObject(namedCaptures);
        }
        replacement = RegExpGetSubstitution(result, S, position, replaceValue,
                                            firstDollarIndex, namedCaptures);
        accumulatedResult += Substring(S, nextSourcePosition,
                                       position - nextSourcePosition) + replacement;
        nextSourcePosition = lastIndex;
        if (matchLength === 0) {
            lastIndex = fullUnicode ? AdvanceStringIndex(S, lastIndex) : lastIndex + 1;
            if (lastIndex > lengthS)
                break;
            lastIndex |= 0;
        }
    }
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpLocalReplaceOpt(rx, S, lengthS, replaceValue
                  )
{
    var lastIndex = ToLength(rx.lastIndex);
    var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
    var globalOrSticky = !!(flags & (0x02 | 0x08));
    if (globalOrSticky) {
        if (lastIndex > lengthS) {
            if (globalOrSticky)
                rx.lastIndex = 0;
            return S;
        }
    } else {
        lastIndex = 0;
    }
    var result = RegExpMatcher(rx, S, lastIndex);
    if (result === null) {
        if (globalOrSticky)
            rx.lastIndex = 0;
        return S;
    }
    ;;
    var matched = result[0];
    var matchLength = matched.length;
    var position = result.index;
    var nextSourcePosition = position + matchLength;
    if (globalOrSticky)
       rx.lastIndex = nextSourcePosition;
    var replacement;
    replacement = replaceValue;
    var accumulatedResult = Substring(S, 0, position) + replacement;
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpLocalReplaceOptShort(rx, S, lengthS, replaceValue
                  )
{
    var lastIndex = ToLength(rx.lastIndex);
    var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
    var globalOrSticky = !!(flags & (0x02 | 0x08));
    if (globalOrSticky) {
        if (lastIndex > lengthS) {
            if (globalOrSticky)
                rx.lastIndex = 0;
            return S;
        }
    } else {
        lastIndex = 0;
    }
    var result = RegExpSearcher(rx, S, lastIndex);
    if (result === -1) {
        if (globalOrSticky)
            rx.lastIndex = 0;
        return S;
    }
    var position = result & 0x7fff;
    var nextSourcePosition = (result >> 15) & 0x7fff;
    if (globalOrSticky)
       rx.lastIndex = nextSourcePosition;
    var replacement;
    replacement = replaceValue;
    var accumulatedResult = Substring(S, 0, position) + replacement;
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpLocalReplaceOptFunc(rx, S, lengthS, replaceValue
                  )
{
    var lastIndex = ToLength(rx.lastIndex);
    var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
    var globalOrSticky = !!(flags & (0x02 | 0x08));
    if (globalOrSticky) {
        if (lastIndex > lengthS) {
            if (globalOrSticky)
                rx.lastIndex = 0;
            return S;
        }
    } else {
        lastIndex = 0;
    }
    var result = RegExpMatcher(rx, S, lastIndex);
    if (result === null) {
        if (globalOrSticky)
            rx.lastIndex = 0;
        return S;
    }
    ;;
    var matched = result[0];
    var matchLength = matched.length;
    var position = result.index;
    var nextSourcePosition = position + matchLength;
    if (globalOrSticky)
       rx.lastIndex = nextSourcePosition;
    var replacement;
    replacement = RegExpGetFunctionalReplacement(result, S, position, replaceValue);
    var accumulatedResult = Substring(S, 0, position) + replacement;
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpLocalReplaceOptSubst(rx, S, lengthS, replaceValue
                   , firstDollarIndex
                  )
{
    var lastIndex = ToLength(rx.lastIndex);
    var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
    var globalOrSticky = !!(flags & (0x02 | 0x08));
    if (globalOrSticky) {
        if (lastIndex > lengthS) {
            if (globalOrSticky)
                rx.lastIndex = 0;
            return S;
        }
    } else {
        lastIndex = 0;
    }
    var result = RegExpMatcher(rx, S, lastIndex);
    if (result === null) {
        if (globalOrSticky)
            rx.lastIndex = 0;
        return S;
    }
    ;;
    var matched = result[0];
    var matchLength = matched.length;
    var position = result.index;
    var nextSourcePosition = position + matchLength;
    if (globalOrSticky)
       rx.lastIndex = nextSourcePosition;
    var replacement;
    var namedCaptures = result.groups;
    if (namedCaptures !== undefined) {
        namedCaptures = ToObject(namedCaptures);
    }
    replacement = RegExpGetSubstitution(result, S, position, replaceValue, firstDollarIndex,
                                        namedCaptures);
    var accumulatedResult = Substring(S, 0, position) + replacement;
    if (nextSourcePosition >= lengthS)
        return accumulatedResult;
    return accumulatedResult + Substring(S, nextSourcePosition, lengthS - nextSourcePosition);
}
function RegExpSearch(string) {
    var rx = this;
    if (!IsObject(rx))
        ThrowTypeError(48, rx === null ? "null" : typeof rx);
    var S = ToString(string);
    var previousLastIndex = rx.lastIndex;
    var lastIndexIsZero = SameValue(previousLastIndex, 0);
    if (!lastIndexIsZero)
        rx.lastIndex = 0;
    if (IsRegExpMethodOptimizable(rx) && S.length < 0x7fff) {
        var result = RegExpSearcher(rx, S, 0);
        if (!lastIndexIsZero) {
            rx.lastIndex = previousLastIndex;
        } else {
            var flags = UnsafeGetInt32FromReservedSlot(rx, 2);
            if (flags & (0x02 | 0x08))
                rx.lastIndex = previousLastIndex;
        }
        if (result === -1)
            return -1;
        return result & 0x7fff;
    }
    return RegExpSearchSlowPath(rx, S, previousLastIndex);
}
function RegExpSearchSlowPath(rx, S, previousLastIndex) {
    var result = RegExpExec(rx, S, false);
    var currentLastIndex = rx.lastIndex;
    if (!SameValue(currentLastIndex, previousLastIndex))
        rx.lastIndex = previousLastIndex;
    if (result === null)
        return -1;
    return result.index;
}
function IsRegExpSplitOptimizable(rx, C) {
    if (!IsRegExpObject(rx))
        return false;
    var RegExpCtor = GetBuiltinConstructor("RegExp");
    if (C !== RegExpCtor)
        return false;
    var RegExpProto = RegExpCtor.prototype;
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpInstanceOptimizable(rx, RegExpProto) &&
           RegExpProto.exec === RegExp_prototype_Exec;
}
function RegExpSplit(string, limit) {
    var rx = this;
    if (!IsObject(rx))
        ThrowTypeError(48, rx === null ? "null" : typeof rx);
    var S = ToString(string);
    var C = SpeciesConstructor(rx, GetBuiltinConstructor("RegExp"));
    var optimizable = IsRegExpSplitOptimizable(rx, C) &&
                      (limit === undefined || typeof limit == "number");
    var flags, unicodeMatching, splitter;
    if (optimizable) {
        flags = UnsafeGetInt32FromReservedSlot(rx, 2);
        unicodeMatching = !!(flags & (0x10));
        if (flags & 0x08) {
            var source = UnsafeGetStringFromReservedSlot(rx, 1);
            splitter = regexp_construct_raw_flags(source, flags & ~0x08);
        } else {
            splitter = rx;
        }
    } else {
        flags = ToString(rx.flags);
        unicodeMatching = callFunction(std_String_includes, flags, "u");
        var newFlags;
        if (callFunction(std_String_includes, flags, "y"))
            newFlags = flags;
        else
            newFlags = flags + "y";
        splitter = new C(rx, newFlags);
    }
    var A = [];
    var lengthA = 0;
    var lim;
    if (limit === undefined)
        lim = 0xffffffff;
    else
        lim = limit >>> 0;
    var p = 0;
    if (lim === 0)
        return A;
    var size = S.length;
    if (size === 0) {
        var z;
        if (optimizable)
            z = RegExpMatcher(splitter, S, 0);
        else
            z = RegExpExec(splitter, S, false);
        if (z !== null)
            return A;
        _DefineDataProperty(A, 0, S);
        return A;
    }
    var q = p;
    while (q < size) {
        var e;
        if (optimizable) {
            z = RegExpMatcher(splitter, S, q);
            if (z === null)
                break;
            q = z.index;
            if (q >= size)
                break;
            e = q + z[0].length;
        } else {
            splitter.lastIndex = q;
            z = RegExpExec(splitter, S, false);
            if (z === null) {
                q = unicodeMatching ? AdvanceStringIndex(S, q) : q + 1;
                continue;
            }
            e = ToLength(splitter.lastIndex);
        }
        if (e === p) {
            q = unicodeMatching ? AdvanceStringIndex(S, q) : q + 1;
            continue;
        }
        _DefineDataProperty(A, lengthA, Substring(S, p, q - p));
        lengthA++;
        if (lengthA === lim)
            return A;
        p = e;
        var numberOfCaptures = std_Math_max(ToLength(z.length) - 1, 0);
        var i = 1;
        while (i <= numberOfCaptures) {
            _DefineDataProperty(A, lengthA, z[i]);
            i++;
            lengthA++;
            if (lengthA === lim)
                return A;
        }
        q = p;
    }
    if (p >= size)
        _DefineDataProperty(A, lengthA, "");
    else
        _DefineDataProperty(A, lengthA, Substring(S, p, size - p));
    return A;
}
function RegExp_prototype_Exec(string) {
    var R = this;
    if (!IsObject(R) || !IsRegExpObject(R))
        return callFunction(CallRegExpMethodIfWrapped, R, string, "RegExp_prototype_Exec");
    var S = ToString(string);
    return RegExpBuiltinExec(R, S, false);
}
function RegExpExec(R, S, forTest) {
    var exec = R.exec;
    if (exec === RegExp_prototype_Exec || !IsCallable(exec)) {
        return RegExpBuiltinExec(R, S, forTest);
    }
    var result = callContentFunction(exec, R, S);
    if (result !== null && !IsObject(result))
        ThrowTypeError(495);
    return forTest ? result !== null : result;
}
function RegExpBuiltinExec(R, S, forTest) {
    if (!IsRegExpObject(R))
        return UnwrapAndCallRegExpBuiltinExec(R, S, forTest);
    var lastIndex = ToLength(R.lastIndex);
    var flags = UnsafeGetInt32FromReservedSlot(R, 2);
    var globalOrSticky = !!(flags & (0x02 | 0x08));
    if (!globalOrSticky) {
        lastIndex = 0;
    } else {
        if (lastIndex > S.length) {
            if (globalOrSticky)
                R.lastIndex = 0;
            return forTest ? false : null;
        }
    }
    if (forTest) {
        var endIndex = RegExpTester(R, S, lastIndex);
        if (endIndex == -1) {
            if (globalOrSticky)
                R.lastIndex = 0;
            return false;
        }
        if (globalOrSticky)
            R.lastIndex = endIndex;
        return true;
    }
    var result = RegExpMatcher(R, S, lastIndex);
    if (result === null) {
        if (globalOrSticky)
            R.lastIndex = 0;
    } else {
        if (globalOrSticky)
            R.lastIndex = result.index + result[0].length;
    }
    return result;
}
function UnwrapAndCallRegExpBuiltinExec(R, S, forTest) {
    return callFunction(CallRegExpMethodIfWrapped, R, S, forTest, "CallRegExpBuiltinExec");
}
function CallRegExpBuiltinExec(S, forTest) {
    return RegExpBuiltinExec(this, S, forTest);
}
function RegExpTest(string) {
    var R = this;
    if (!IsObject(R))
        ThrowTypeError(48, R === null ? "null" : typeof R);
    var S = ToString(string);
    return RegExpExec(R, S, true);
}
function $RegExpSpecies() {
    return this;
}
_SetCanonicalName($RegExpSpecies, "get [Symbol.species]");
function IsRegExpMatchAllOptimizable(rx, C) {
    if (!IsRegExpObject(rx))
        return false;
    var RegExpCtor = GetBuiltinConstructor("RegExp");
    if (C !== RegExpCtor)
        return false;
    var RegExpProto = RegExpCtor.prototype;
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpInstanceOptimizable(rx, RegExpProto);
}
function RegExpMatchAll(string) {
    var rx = this;
    if (!IsObject(rx))
        ThrowTypeError(48, rx === null ? "null" : typeof rx);
    var str = ToString(string);
    var C = SpeciesConstructor(rx, GetBuiltinConstructor("RegExp"));
    var source, flags, matcher, lastIndex;
    if (IsRegExpMatchAllOptimizable(rx, C)) {
        source = UnsafeGetStringFromReservedSlot(rx, 1);
        flags = UnsafeGetInt32FromReservedSlot(rx, 2);
        matcher = rx;
        lastIndex = ToLength(rx.lastIndex);
    } else {
        source = "";
        flags = ToString(rx.flags);
        matcher = new C(rx, flags);
        matcher.lastIndex = ToLength(rx.lastIndex);
        flags = (callFunction(std_String_includes, flags, "g") ? 0x02 : 0) |
                (callFunction(std_String_includes, flags, "u") ? 0x10 : 0);
        lastIndex = -2;
    }
    return CreateRegExpStringIterator(matcher, str, source, flags, lastIndex);
}
function CreateRegExpStringIterator(regexp, string, source, flags, lastIndex) {
    ;;
    ;;
    ;;
    ;;
    var iterator = NewRegExpStringIterator();
    UnsafeSetReservedSlot(iterator, 0, regexp);
    UnsafeSetReservedSlot(iterator, 1, string);
    UnsafeSetReservedSlot(iterator, 2, source);
    UnsafeSetReservedSlot(iterator, 3, flags | 0);
    UnsafeSetReservedSlot(iterator, 4, lastIndex);
    return iterator;
}
function IsRegExpStringIteratorNextOptimizable() {
    var RegExpProto = GetBuiltinPrototype("RegExp");
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpProto.exec === RegExp_prototype_Exec;
}
function RegExpStringIteratorNext() {
    var obj;
    if (!IsObject(this) || (obj = GuardToRegExpStringIterator(this)) === null) {
        return callFunction(CallRegExpStringIteratorMethodIfWrapped, this,
                            "RegExpStringIteratorNext");
    }
    var result = { value: undefined, done: false };
    var lastIndex = UnsafeGetReservedSlot(obj, 4);
    if (lastIndex === -1) {
        result.done = true;
        return result;
    }
    var regexp = UnsafeGetObjectFromReservedSlot(obj, 0);
    var string = UnsafeGetStringFromReservedSlot(obj, 1);
    var flags = UnsafeGetInt32FromReservedSlot(obj, 3);
    var global = !!(flags & 0x02);
    var fullUnicode = !!(flags & 0x10);
    if (lastIndex >= 0) {
        ;;
        var source = UnsafeGetStringFromReservedSlot(obj, 2);
        if (IsRegExpStringIteratorNextOptimizable() &&
            UnsafeGetStringFromReservedSlot(regexp, 1) === source &&
            UnsafeGetInt32FromReservedSlot(regexp, 2) === flags)
        {
            var globalOrSticky = !!(flags & (0x02 | 0x08));
            if (!globalOrSticky)
                lastIndex = 0;
            var match = (lastIndex <= string.length)
                        ? RegExpMatcher(regexp, string, lastIndex)
                        : null;
            if (match === null) {
                UnsafeSetReservedSlot(obj, 4,
                                      -1);
                result.done = true;
                return result;
            }
            if (global) {
                var matchLength = match[0].length;
                lastIndex = match.index + matchLength;
                if (matchLength === 0) {
                    lastIndex = fullUnicode ? AdvanceStringIndex(string, lastIndex) : lastIndex + 1;
                }
                UnsafeSetReservedSlot(obj, 4, lastIndex);
            } else {
                UnsafeSetReservedSlot(obj, 4,
                                      -1);
            }
            result.value = match;
            return result;
        }
        regexp = regexp_construct_raw_flags(source, flags);
        regexp.lastIndex = lastIndex;
        UnsafeSetReservedSlot(obj, 0, regexp);
        UnsafeSetReservedSlot(obj, 4,
                              -2);
    }
    var match = RegExpExec(regexp, string, false);
    if (match === null) {
        UnsafeSetReservedSlot(obj, 4,
                              -1);
        result.done = true;
        return result;
    }
    if (global) {
        var matchStr = ToString(match[0]);
        if (matchStr.length === 0) {
            var thisIndex = ToLength(regexp.lastIndex);
            var nextIndex = fullUnicode ? AdvanceStringIndex(string, thisIndex) : thisIndex + 1;
            regexp.lastIndex = nextIndex;
        }
    } else {
        UnsafeSetReservedSlot(obj, 4,
                              -1);
    }
    result.value = match;
    return result;
}
function IsRegExp(argument) {
    if (!IsObject(argument)) {
        return false;
    }
    var matcher = argument[std_match];
    if (matcher !== undefined) {
        return !!matcher;
    }
    return IsPossiblyWrappedRegExpObject(argument);
}
function StringProtoHasNoMatch() {
    var ObjectProto = GetBuiltinPrototype("Object");
    var StringProto = GetBuiltinPrototype("String");
    if (!ObjectHasPrototype(StringProto, ObjectProto))
        return false;
    return !(std_match in StringProto);
}
function IsStringMatchOptimizable() {
    var RegExpProto = GetBuiltinPrototype("RegExp");
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpProto.exec === RegExp_prototype_Exec &&
           RegExpProto[std_match] === RegExpMatch;
}
function ThrowIncompatibleMethod(name, thisv) {
    ThrowTypeError(3, "String", name, ToString(thisv));
}
function String_match(regexp) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("match", this);
    var isPatternString = (typeof regexp === "string");
    if (!(isPatternString && StringProtoHasNoMatch()) && regexp !== undefined && regexp !== null) {
        var matcher = GetMethod(regexp, std_match);
        if (matcher !== undefined)
            return callContentFunction(matcher, regexp, this);
    }
    var S = ToString(this);
    if (isPatternString && IsStringMatchOptimizable()) {
        var flatResult = FlatStringMatch(S, regexp);
        if (flatResult !== undefined)
            return flatResult;
    }
    var rx = RegExpCreate(regexp);
    if (IsStringMatchOptimizable())
        return RegExpMatcher(rx, S, 0);
    return callContentFunction(GetMethod(rx, std_match), rx, S);
}
function String_matchAll(regexp) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("matchAll", this);
    if (regexp !== undefined && regexp !== null) {
        if (IsRegExp(regexp)) {
            var flags = regexp.flags;
            if (flags === undefined || flags === null) {
                ThrowTypeError(99);
            }
            if (!callFunction(std_String_includes, ToString(flags), "g")) {
                ThrowTypeError(100, "matchAll");
            }
        }
        var matcher = GetMethod(regexp, std_matchAll);
        if (matcher !== undefined)
            return callContentFunction(matcher, regexp, this);
    }
    var string = ToString(this);
    var rx = RegExpCreate(regexp, "g");
    return callContentFunction(GetMethod(rx, std_matchAll), rx, string);
}
function String_pad(maxLength, fillString, padEnd) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod(padEnd ? "padEnd" : "padStart", this);
    let str = ToString(this);
    let intMaxLength = ToLength(maxLength);
    let strLen = str.length;
    if (intMaxLength <= strLen)
        return str;
    ;;
    let filler = ToString(fillString);
    if (filler === "")
        return str;
    if (intMaxLength > ((1 << 30) - 2))
        ThrowRangeError(98);
    let fillLen = intMaxLength - strLen;
    let truncatedStringFiller = callFunction(String_repeat, filler,
                                             (fillLen / filler.length) | 0);
    truncatedStringFiller += Substring(filler, 0, fillLen % filler.length);
    if (padEnd === true)
        return str + truncatedStringFiller;
    return truncatedStringFiller + str;
}
function String_pad_start(maxLength, fillString = " ") {
    return callFunction(String_pad, this, maxLength, fillString, false);
}
function String_pad_end(maxLength, fillString = " ") {
    return callFunction(String_pad, this, maxLength, fillString, true);
}
function StringProtoHasNoReplace() {
    var ObjectProto = GetBuiltinPrototype("Object");
    var StringProto = GetBuiltinPrototype("String");
    if (!ObjectHasPrototype(StringProto, ObjectProto))
        return false;
    return !(std_replace in StringProto);
}
function Substring(str, from, length) {
    ;;
    ;;
    ;;
    return SubstringKernel(str, from | 0, length | 0);
}
function String_replace(searchValue, replaceValue) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("replace", this);
    if (!(typeof searchValue === "string" && StringProtoHasNoReplace()) &&
        searchValue !== undefined && searchValue !== null)
    {
        var replacer = GetMethod(searchValue, std_replace);
        if (replacer !== undefined)
            return callContentFunction(replacer, searchValue, this, replaceValue);
    }
    var string = ToString(this);
    var searchString = ToString(searchValue);
    if (typeof replaceValue === "string") {
        return StringReplaceString(string, searchString, replaceValue);
    }
    if (!IsCallable(replaceValue)) {
        return StringReplaceString(string, searchString, ToString(replaceValue));
    }
    var pos = callFunction(std_String_indexOf, string, searchString);
    if (pos === -1)
        return string;
    var replStr = ToString(callContentFunction(replaceValue, undefined, searchString, pos, string));
    var tailPos = pos + searchString.length;
    var newString;
    if (pos === 0)
        newString = "";
    else
        newString = Substring(string, 0, pos);
    newString += replStr;
    var stringLength = string.length;
    if (tailPos < stringLength)
        newString += Substring(string, tailPos, stringLength - tailPos);
    return newString;
}
function String_replaceAll(searchValue, replaceValue) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("replaceAll", this);
    if (searchValue !== undefined && searchValue !== null) {
        if (IsRegExp(searchValue)) {
            var flags = searchValue.flags;
            if (flags === undefined || flags === null) {
                ThrowTypeError(99);
            }
            if (!callFunction(std_String_includes, ToString(flags), "g")) {
                ThrowTypeError(100, "replaceAll");
            }
        }
        var replacer = GetMethod(searchValue, std_replace);
        if (replacer !== undefined) {
            return callContentFunction(replacer, searchValue, this, replaceValue);
        }
    }
    var string = ToString(this);
    var searchString = ToString(searchValue);
    if (!IsCallable(replaceValue)) {
        return StringReplaceAllString(string, searchString, ToString(replaceValue));
    }
    var searchLength = searchString.length;
    var advanceBy = std_Math_max(1, searchLength);
    var endOfLastMatch = 0;
    var result = "";
    var position = 0;
    while (true) {
        var nextPosition = callFunction(std_String_indexOf, string, searchString, position);
        if (nextPosition < position) {
            break;
        }
        position = nextPosition;
        var replacement = ToString(callContentFunction(replaceValue, undefined, searchString,
                                                       position, string));
        var stringSlice = Substring(string, endOfLastMatch, position - endOfLastMatch);
        result += stringSlice + replacement;
        endOfLastMatch = position + searchLength;
        position += advanceBy;
    }
    if (endOfLastMatch < string.length) {
        result += Substring(string, endOfLastMatch, string.length - endOfLastMatch);
    }
    return result;
}
function StringProtoHasNoSearch() {
    var ObjectProto = GetBuiltinPrototype("Object");
    var StringProto = GetBuiltinPrototype("String");
    if (!ObjectHasPrototype(StringProto, ObjectProto))
        return false;
    return !(std_search in StringProto);
}
function IsStringSearchOptimizable() {
    var RegExpProto = GetBuiltinPrototype("RegExp");
    return RegExpPrototypeOptimizable(RegExpProto) &&
           RegExpProto.exec === RegExp_prototype_Exec &&
           RegExpProto[std_search] === RegExpSearch;
}
function String_search(regexp) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("search", this);
    var isPatternString = (typeof regexp === "string");
    if (!(isPatternString && StringProtoHasNoSearch()) && regexp !== undefined && regexp !== null) {
        var searcher = GetMethod(regexp, std_search);
        if (searcher !== undefined)
            return callContentFunction(searcher, regexp, this);
    }
    var string = ToString(this);
    if (isPatternString && IsStringSearchOptimizable()) {
        var flatResult = FlatStringSearch(string, regexp);
        if (flatResult !== -2)
            return flatResult;
    }
    var rx = RegExpCreate(regexp);
    return callContentFunction(GetMethod(rx, std_search), rx, string);
}
function StringProtoHasNoSplit() {
    var ObjectProto = GetBuiltinPrototype("Object");
    var StringProto = GetBuiltinPrototype("String");
    if (!ObjectHasPrototype(StringProto, ObjectProto))
        return false;
    return !(std_split in StringProto);
}
function String_split(separator, limit) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("split", this);
    if (typeof this === "string") {
        if (StringProtoHasNoSplit()) {
            if (typeof separator === "string") {
                if (limit === undefined) {
                    return StringSplitString(this, separator);
                }
            }
        }
    }
    if (!(typeof separator == "string" && StringProtoHasNoSplit()) &&
        separator !== undefined && separator !== null)
    {
        var splitter = GetMethod(separator, std_split);
        if (splitter !== undefined)
            return callContentFunction(splitter, separator, this, limit);
    }
    var S = ToString(this);
    var R;
    if (limit !== undefined) {
        var lim = limit >>> 0;
        R = ToString(separator);
        if (lim === 0)
            return [];
        if (separator === undefined)
            return [S];
        return StringSplitStringLimit(S, R, lim);
    }
    R = ToString(separator);
    if (separator === undefined)
        return [S];
    return StringSplitString(S, R);
}
function String_substring(start, end) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("substring", this);
    var str = ToString(this);
    var len = str.length;
    var intStart = ToInteger(start);
    var intEnd = (end === undefined) ? len : ToInteger(end);
    var finalStart = std_Math_min(std_Math_max(intStart, 0), len);
    var finalEnd = std_Math_min(std_Math_max(intEnd, 0), len);
    var from, to;
    if (finalStart < finalEnd) {
        from = finalStart;
        to = finalEnd;
    } else {
        from = finalEnd;
        to = finalStart;
    }
    return SubstringKernel(str, from | 0, (to - from) | 0);
}
_SetIsInlinableLargeFunction(String_substring);
function String_substr(start, length) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("substr", this);
    var str = ToString(this);
    var intStart = ToInteger(start);
    var size = str.length;
    var end = (length === undefined) ? size : ToInteger(length);
    if (intStart < 0)
        intStart = std_Math_max(intStart + size, 0);
    var resultLength = std_Math_min(std_Math_max(end, 0), size - intStart);
    if (resultLength <= 0)
        return "";
    return SubstringKernel(str, intStart | 0, resultLength | 0);
}
_SetIsInlinableLargeFunction(String_substr);
function String_concat(arg1) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("concat", this);
    var str = ToString(this);
    if (arguments.length === 0) {
        return str;
    }
    if (arguments.length === 1) {
        return str + ToString(arguments[0]);
    }
    if (arguments.length === 2) {
        return str + ToString(arguments[0]) + ToString(arguments[1]);
    }
    var result = str;
    for (var i = 0; i < arguments.length; i++) {
        var nextString = ToString(arguments[i]);
        result += nextString;
    }
    return result;
}
function String_slice(start, end) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("slice", this);
    var str = ToString(this);
    var len = str.length;
    var intStart = ToInteger(start);
    var intEnd = (end === undefined) ? len : ToInteger(end);
    var from = (intStart < 0) ? std_Math_max(len + intStart, 0) : std_Math_min(intStart, len);
    var to = (intEnd < 0) ? std_Math_max(len + intEnd, 0) : std_Math_min(intEnd, len);
    var span = std_Math_max(to - from, 0);
    return SubstringKernel(str, from | 0, span | 0);
}
_SetIsInlinableLargeFunction(String_slice);
function String_codePointAt(pos) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("codePointAt", this);
    var S = ToString(this);
    var position = ToInteger(pos);
    var size = S.length;
    if (position < 0 || position >= size)
        return undefined;
    var first = callFunction(std_String_charCodeAt, S, position);
    if (first < 0xD800 || first > 0xDBFF || position + 1 === size)
        return first;
    var second = callFunction(std_String_charCodeAt, S, position + 1);
    if (second < 0xDC00 || second > 0xDFFF)
        return first;
    return (first - 0xD800) * 0x400 + (second - 0xDC00) + 0x10000;
}
function String_repeat(count) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("repeat", this);
    var S = ToString(this);
    var n = ToInteger(count);
    if (n < 0)
        ThrowRangeError(96);
    if (!(n * S.length <= ((1 << 30) - 2)))
        ThrowRangeError(98);
    ;;
    ;;
    n = n & (((1 << 30) - 2) + 1);
    var T = "";
    for (;;) {
        if (n & 1)
            T += S;
        n >>= 1;
        if (n)
            S += S;
        else
            break;
    }
    return T;
}
function String_iterator() {
    if (this === undefined || this === null) {
        ThrowTypeError(4, "String", "Symbol.iterator",
                       ToString(this));
    }
    var S = ToString(this);
    var iterator = NewStringIterator();
    UnsafeSetReservedSlot(iterator, 0, S);
    UnsafeSetReservedSlot(iterator, 1, 0);
    return iterator;
}
function StringIteratorNext() {
    var obj;
    if (!IsObject(this) || (obj = GuardToStringIterator(this)) === null) {
        return callFunction(CallStringIteratorMethodIfWrapped, this,
                            "StringIteratorNext");
    }
    var S = UnsafeGetStringFromReservedSlot(obj, 0);
    var index = UnsafeGetInt32FromReservedSlot(obj, 1);
    var size = S.length;
    var result = { value: undefined, done: false };
    if (index >= size) {
        result.done = true;
        return result;
    }
    var charCount = 1;
    var first = callFunction(std_String_charCodeAt, S, index);
    if (first >= 0xD800 && first <= 0xDBFF && index + 1 < size) {
        var second = callFunction(std_String_charCodeAt, S, index + 1);
        if (second >= 0xDC00 && second <= 0xDFFF) {
            first = (first - 0xD800) * 0x400 + (second - 0xDC00) + 0x10000;
            charCount = 2;
        }
    }
    UnsafeSetReservedSlot(obj, 1, index + charCount);
    result.value = callFunction(std_String_fromCodePoint, null, first & 0x1fffff);
    return result;
}
var collatorCache = new Record();
function String_localeCompare(that) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("localeCompare", this);
    var S = ToString(this);
    var That = ToString(that);
    var locales = arguments.length > 1 ? arguments[1] : undefined;
    var options = arguments.length > 2 ? arguments[2] : undefined;
    var collator;
    if (locales === undefined && options === undefined) {
        if (!IsRuntimeDefaultLocale(collatorCache.runtimeDefaultLocale)) {
            collatorCache.collator = intl_Collator(locales, options);
            collatorCache.runtimeDefaultLocale = RuntimeDefaultLocale();
        }
        collator = collatorCache.collator;
    } else {
        collator = intl_Collator(locales, options);
    }
    return intl_CompareStrings(collator, S, That);
}
function String_toLocaleLowerCase() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("toLocaleLowerCase", this);
    var string = ToString(this);
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var requestedLocale;
    if (locales === undefined) {
        requestedLocale = undefined;
    } else if (typeof locales === "string") {
        requestedLocale = intl_ValidateAndCanonicalizeLanguageTag(locales, false);
    } else {
        var requestedLocales = CanonicalizeLocaleList(locales);
        requestedLocale = requestedLocales.length > 0 ? requestedLocales[0] : undefined;
    }
    if (string.length === 0)
        return "";
    if (requestedLocale === undefined)
        requestedLocale = DefaultLocale();
    return intl_toLocaleLowerCase(string, requestedLocale);
}
function String_toLocaleUpperCase() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("toLocaleUpperCase", this);
    var string = ToString(this);
    var locales = arguments.length > 0 ? arguments[0] : undefined;
    var requestedLocale;
    if (locales === undefined) {
        requestedLocale = undefined;
    } else if (typeof locales === "string") {
        requestedLocale = intl_ValidateAndCanonicalizeLanguageTag(locales, false);
    } else {
        var requestedLocales = CanonicalizeLocaleList(locales);
        requestedLocale = requestedLocales.length > 0 ? requestedLocales[0] : undefined;
    }
    if (string.length === 0)
        return "";
    if (requestedLocale === undefined)
        requestedLocale = DefaultLocale();
    return intl_toLocaleUpperCase(string, requestedLocale);
}
function String_static_raw(callSite ) {
    var cooked = ToObject(callSite);
    var raw = ToObject(cooked.raw);
    var literalSegments = ToLength(raw.length);
    if (literalSegments === 0)
        return "";
    if (literalSegments === 1)
        return ToString(raw[0]);
    var resultString = ToString(raw[0]);
    for (var nextIndex = 1; nextIndex < literalSegments; nextIndex++) {
        if (nextIndex < arguments.length)
            resultString += ToString(arguments[nextIndex]);
        resultString += ToString(raw[nextIndex]);
    }
    return resultString;
}
function String_at(index) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("at", this);
    var string = ToString(this);
    var len = string.length;
    var relativeIndex = ToInteger(index);
    var k;
    if (relativeIndex >= 0) {
        k = relativeIndex;
    } else {
        k = len + relativeIndex;
    }
    if (k < 0 || k >= len) {
        return undefined;
    }
    return string[k];
}
function String_big() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("big", this);
    return "<big>" + ToString(this) + "</big>";
}
function String_blink() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("blink", this);
    return "<blink>" + ToString(this) + "</blink>";
}
function String_bold() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("bold", this);
    return "<b>" + ToString(this) + "</b>";
}
function String_fixed() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("fixed", this);
    return "<tt>" + ToString(this) + "</tt>";
}
function String_italics() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("italics", this);
    return "<i>" + ToString(this) + "</i>";
}
function String_small() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("small", this);
    return "<small>" + ToString(this) + "</small>";
}
function String_strike() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("strike", this);
    return "<strike>" + ToString(this) + "</strike>";
}
function String_sub() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("sub", this);
    return "<sub>" + ToString(this) + "</sub>";
}
function String_sup() {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("sup", this);
    return "<sup>" + ToString(this) + "</sup>";
}
function EscapeAttributeValue(v) {
    var inputStr = ToString(v);
    return StringReplaceAllString(inputStr, '"', "&quot;");
}
function String_anchor(name) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("anchor", this);
    var S = ToString(this);
    return '<a name="' + EscapeAttributeValue(name) + '">' + S + "</a>";
}
function String_fontcolor(color) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("fontcolor", this);
    var S = ToString(this);
    return '<font color="' + EscapeAttributeValue(color) + '">' + S + "</font>";
}
function String_fontsize(size) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("fontsize", this);
    var S = ToString(this);
    return '<font size="' + EscapeAttributeValue(size) + '">' + S + "</font>";
}
function String_link(url) {
    if (this === undefined || this === null)
        ThrowIncompatibleMethod("link", this);
    var S = ToString(this);
    return '<a href="' + EscapeAttributeValue(url) + '">' + S + "</a>";
}
function SetConstructorInit(iterable) {
    var set = this;
    var adder = set.add;
    if (!IsCallable(adder))
        ThrowTypeError(10, typeof adder);
    for (var nextValue of allowContentIter(iterable))
        callContentFunction(adder, set, nextValue);
}
function SetForEach(callbackfn, thisArg = undefined) {
    var S = this;
    if (!IsObject(S) || (S = GuardToSetObject(S)) === null)
        return callFunction(CallSetMethodIfWrapped, this, callbackfn, thisArg, "SetForEach");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var values = callFunction(std_Set_values, S);
    var setIterationResult = setIteratorTemp.setIterationResult;
    if (!setIterationResult)
        setIterationResult = setIteratorTemp.setIterationResult = _CreateSetIterationResult();
    while (true) {
        var done = _GetNextSetEntryForIterator(values, setIterationResult);
        if (done)
            break;
        var value = setIterationResult[0];
        setIterationResult[0] = null;
        callContentFunction(callbackfn, thisArg, value, value, S);
    }
}
function $SetSpecies() {
    return this;
}
_SetCanonicalName($SetSpecies, "get [Symbol.species]");
var setIteratorTemp = { setIterationResult: null };
function SetIteratorNext() {
    var O = this;
    if (!IsObject(O) || (O = GuardToSetIterator(O)) === null)
        return callFunction(CallSetIteratorMethodIfWrapped, this, "SetIteratorNext");
    var setIterationResult = setIteratorTemp.setIterationResult;
    if (!setIterationResult)
        setIterationResult = setIteratorTemp.setIterationResult = _CreateSetIterationResult();
    var retVal = {value: undefined, done: true};
    var done = _GetNextSetEntryForIterator(O, setIterationResult);
    if (!done) {
        var itemKind = UnsafeGetInt32FromReservedSlot(O, 2);
        var result;
        if (itemKind === 1) {
            result = setIterationResult[0];
        } else {
            ;;
            result = [setIterationResult[0], setIterationResult[0]];
        }
        setIterationResult[0] = null;
        retVal.value = result;
        retVal.done = false;
    }
    return retVal;
}
function CountingSort(array, len, signed, comparefn) {
    ;;
    if (len < 128) {
        QuickSort(array, len, comparefn);
        return array;
    }
    var min = 0;
    if (signed) {
        min = -128;
    }
    var buffer = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    ];
    for (var i = 0; i < len; i++) {
        var val = array[i];
        buffer[val - min]++;
    }
    var val = -1;
    for (var i = 0; i < len;) {
        var j;
        do {
            j = buffer[++val];
        } while (j === 0);
        for (; j > 0; j--)
            array[i++] = val + min;
    }
    return array;
}
function ByteAtCol(x, pos) {
    return (x >> (pos * 8)) & 0xFF;
}
function SortByColumn(array, len, aux, col, counts) {
    const R = 256;
    ;;
    for (let r = 0; r < R + 1; r++) {
        counts[r] = 0;
    }
    for (let i = 0; i < len; i++) {
        let val = array[i];
        let b = ByteAtCol(val, col);
        counts[b + 1]++;
    }
    for (let r = 0; r < R; r++) {
        counts[r + 1] += counts[r];
    }
    for (let i = 0; i < len; i++) {
        let val = array[i];
        let b = ByteAtCol(val, col);
        aux[counts[b]++] = val;
    }
    for (let i = 0; i < len; i++) {
        array[i] = aux[i];
    }
}
function RadixSort(array, len, buffer, nbytes, signed, floating, comparefn) {
    ;;
    if (len < 512) {
        QuickSort(array, len, comparefn);
        return array;
    }
    let aux = [];
    for (let i = 0; i < len; i++)
        _DefineDataProperty(aux, i, 0);
    let view = array;
    let signMask = 1 << nbytes * 8 - 1;
    if (floating) {
        if (buffer === null) {
            buffer = callFunction(std_TypedArray_buffer, array);
            ;;
        }
        let offset = IsTypedArray(array)
                     ? TypedArrayByteOffset(array)
                     : callFunction(CallTypedArrayMethodIfWrapped, array,
                                    "TypedArrayByteOffsetMethod");
        view = new Int32Array(buffer, offset, len);
        for (let i = 0; i < len; i++) {
            if (view[i] & signMask) {
                if ((view[i] & 0x7F800000) !== 0x7F800000 || (view[i] & 0x007FFFFF) === 0) {
                    view[i] ^= 0xFFFFFFFF;
                }
            } else {
                view[i] ^= signMask;
            }
        }
    } else if (signed) {
        for (let i = 0; i < len; i++) {
            view[i] ^= signMask;
        }
    }
    let counts = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,
    ];
    for (let col = 0; col < nbytes; col++) {
        SortByColumn(view, len, aux, col, counts);
    }
    if (floating) {
        for (let i = 0; i < len; i++) {
            if (view[i] & signMask) {
                view[i] ^= signMask;
            } else {
                view[i] ^= 0xFFFFFFFF;
            }
        }
    } else if (signed) {
        for (let i = 0; i < len; i++) {
            view[i] ^= signMask;
        }
    }
    return array;
}
function InsertionSort(array, from, to, comparefn) {
    let item, swap, i, j;
    for (i = from + 1; i <= to; i++) {
        item = array[i];
        for (j = i - 1; j >= from; j--) {
            swap = array[j];
            if (comparefn(swap, item) <= 0)
                break;
            array[j + 1] = swap;
        }
        array[j + 1] = item;
    }
}
function SwapArrayElements(array, i, j) {
    var swap = array[i];
    array[i] = array[j];
    array[j] = swap;
}
function Merge(list, out, start, mid, end, comparefn) {
    if (mid >= end || comparefn(list[mid], list[mid + 1]) <= 0) {
        for (var i = start; i <= end; i++) {
            _DefineDataProperty(out, i, list[i]);
        }
        return;
    }
    var i = start;
    var j = mid + 1;
    var k = start;
    while (i <= mid && j <= end) {
        var lvalue = list[i];
        var rvalue = list[j];
        if (comparefn(lvalue, rvalue) <= 0) {
            _DefineDataProperty(out, k++, lvalue);
            i++;
        } else {
            _DefineDataProperty(out, k++, rvalue);
            j++;
        }
    }
    while (i <= mid) {
        _DefineDataProperty(out, k++, list[i++]);
    }
    while (j <= end) {
        _DefineDataProperty(out, k++, list[j++]);
    }
}
function MoveHoles(sparse, sparseLen, dense, denseLen) {
    for (var i = 0; i < denseLen; i++)
        sparse[i] = dense[i];
    for (var j = denseLen; j < sparseLen; j++)
        delete sparse[j];
}
function MergeSort(array, len, comparefn) {
    var denseList = [];
    var denseLen = 0;
    for (var i = 0; i < len; i++) {
        if (i in array)
            _DefineDataProperty(denseList, denseLen++, array[i]);
    }
    if (denseLen < 1)
        return array;
    if (denseLen < 24) {
        InsertionSort(denseList, 0, denseLen - 1, comparefn);
        MoveHoles(array, len, denseList, denseLen);
        return array;
    }
    var lBuffer = denseList;
    var rBuffer = [];
    var windowSize = 4;
    for (var start = 0; start < denseLen - 1; start += windowSize) {
        var end = std_Math_min(start + windowSize - 1, denseLen - 1);
        InsertionSort(lBuffer, start, end, comparefn);
    }
    for (; windowSize < denseLen; windowSize = 2 * windowSize) {
        for (var start = 0; start < denseLen; start += 2 * windowSize) {
            var mid = start + windowSize - 1;
            var end = std_Math_min(start + 2 * windowSize - 1, denseLen - 1);
            Merge(lBuffer, rBuffer, start, mid, end, comparefn);
        }
        var swap = lBuffer;
        lBuffer = rBuffer;
        rBuffer = swap;
    }
    MoveHoles(array, len, lBuffer, denseLen);
    return array;
}
function MergeTypedArray(list, out, start, mid, end, comparefn) {
    if (mid >= end || comparefn(list[mid], list[mid + 1]) <= 0) {
        for (var i = start; i <= end; i++) {
            out[i] = list[i];
        }
        return;
    }
    var i = start;
    var j = mid + 1;
    var k = start;
    while (i <= mid && j <= end) {
        var lvalue = list[i];
        var rvalue = list[j];
        if (comparefn(lvalue, rvalue) <= 0) {
            out[k++] = lvalue;
            i++;
        } else {
            out[k++] = rvalue;
            j++;
        }
    }
    while (i <= mid) {
        out[k++] = list[i++];
    }
    while (j <= end) {
        out[k++] = list[j++];
    }
}
function MergeSortTypedArray(array, len, comparefn) {
    ;;
    if (len < 8) {
        InsertionSort(array, 0, len - 1, comparefn);
        return array;
    }
    var C = _ConstructorForTypedArray(array);
    var lBuffer = array;
    var rBuffer = new C(len);
    var windowSize = 4;
    for (var start = 0; start < len - 1; start += windowSize) {
        var end = std_Math_min(start + windowSize - 1, len - 1);
        InsertionSort(lBuffer, start, end, comparefn);
    }
    for (; windowSize < len; windowSize = 2 * windowSize) {
        for (var start = 0; start < len; start += 2 * windowSize) {
            var mid = start + windowSize - 1;
            var end = std_Math_min(start + 2 * windowSize - 1, len - 1);
            MergeTypedArray(lBuffer, rBuffer, start, mid, end, comparefn);
        }
        var swap = lBuffer;
        lBuffer = rBuffer;
        rBuffer = swap;
    }
    if (lBuffer !== array) {
        for (var i = 0; i < len; i++) {
            array[i] = lBuffer[i];
        }
    }
    return array;
}
function Partition(array, from, to, comparefn) {
    ;;
    var medianIndex = from + ((to - from) >> 1);
    var i = from + 1;
    var j = to;
    SwapArrayElements(array, medianIndex, i);
    if (comparefn(array[from], array[to]) > 0)
        SwapArrayElements(array, from, to);
    if (comparefn(array[i], array[to]) > 0)
        SwapArrayElements(array, i, to);
    if (comparefn(array[from], array[i]) > 0)
        SwapArrayElements(array, from, i);
    var pivotIndex = i;
    for (;;) {
        do i++; while (comparefn(array[i], array[pivotIndex]) < 0);
        do j--; while (comparefn(array[j], array[pivotIndex]) > 0);
        if (i > j)
            break;
        SwapArrayElements(array, i, j);
    }
    SwapArrayElements(array, pivotIndex, j);
    return j;
}
function QuickSort(array, len, comparefn) {
    ;;
    var stack = new List();
    var top = 0;
    var start = 0;
    var end = len - 1;
    var pivotIndex, leftLen, rightLen;
    for (;;) {
        if (end - start <= 23) {
            InsertionSort(array, start, end, comparefn);
            if (top < 1)
                break;
            end = stack[--top];
            start = stack[--top];
        } else {
            pivotIndex = Partition(array, start, end, comparefn);
            leftLen = (pivotIndex - 1) - start;
            rightLen = end - (pivotIndex + 1);
            if (rightLen > leftLen) {
                stack[top++] = start;
                stack[top++] = pivotIndex - 1;
                start = pivotIndex + 1;
            } else {
                stack[top++] = pivotIndex + 1;
                stack[top++] = end;
                end = pivotIndex - 1;
            }
        }
    }
    return array;
}
function ViewedArrayBufferIfReified(tarray) {
    ;;
    var buf = UnsafeGetReservedSlot(tarray, 0);
    ;;
    return buf;
}
function IsDetachedBuffer(buffer) {
    if (buffer === null)
        return false;
    ;;
    if ((buffer = GuardToArrayBuffer(buffer)) === null)
        return false;
    var flags = UnsafeGetInt32FromReservedSlot(buffer, 3);
    return (flags & 0x8) !== 0;
}
function TypedArrayLengthMethod() {
    return TypedArrayLength(this);
}
function TypedArrayByteOffsetMethod() {
    return TypedArrayByteOffset(this);
}
function GetAttachedArrayBuffer(tarray) {
    var buffer = ViewedArrayBufferIfReified(tarray);
    if (IsDetachedBuffer(buffer))
        ThrowTypeError(530);
    return buffer;
}
function GetAttachedArrayBufferMethod() {
    return GetAttachedArrayBuffer(this);
}
function IsTypedArrayEnsuringArrayBuffer(arg) {
    if (IsObject(arg) && IsTypedArray(arg)) {
        GetAttachedArrayBuffer(arg);
        return true;
    }
    callFunction(CallTypedArrayMethodIfWrapped, arg, "GetAttachedArrayBufferMethod");
    return false;
}
function TypedArraySpeciesConstructor(obj) {
    ;;
    var ctor = obj.constructor;
    if (ctor === undefined)
        return _ConstructorForTypedArray(obj);
    if (!IsObject(ctor))
        ThrowTypeError(48, "object's 'constructor' property");
    var s = ctor[std_species];
    if (s === undefined || s === null)
        return _ConstructorForTypedArray(obj);
    if (IsConstructor(s))
        return s;
    ThrowTypeError(11, "@@species property of object's constructor");
}
function ValidateTypedArray(obj) {
    if (IsObject(obj)) {
        if (IsTypedArray(obj)) {
            GetAttachedArrayBuffer(obj);
            return true;
        }
        if (IsPossiblyWrappedTypedArray(obj)) {
            if (PossiblyWrappedTypedArrayHasDetachedBuffer(obj))
                ThrowTypeError(530);
            return false;
        }
    }
    ThrowTypeError(533);
}
function TypedArrayCreateWithLength(constructor, length) {
    var newTypedArray = new constructor(length);
    var isTypedArray = ValidateTypedArray(newTypedArray);
    var len;
    if (isTypedArray) {
        len = TypedArrayLength(newTypedArray);
    } else {
        len = callFunction(CallTypedArrayMethodIfWrapped, newTypedArray,
                           "TypedArrayLengthMethod");
    }
    if (len < length)
        ThrowTypeError(534, length, len);
    return newTypedArray;
}
function TypedArrayCreateWithBuffer(constructor, buffer, byteOffset, length) {
    var newTypedArray = new constructor(buffer, byteOffset, length);
    ValidateTypedArray(newTypedArray);
    return newTypedArray;
}
function TypedArraySpeciesCreateWithLength(exemplar, length) {
    var C = TypedArraySpeciesConstructor(exemplar);
    return TypedArrayCreateWithLength(C, length);
}
function TypedArraySpeciesCreateWithBuffer(exemplar, buffer, byteOffset, length) {
    var C = TypedArraySpeciesConstructor(exemplar);
    return TypedArrayCreateWithBuffer(C, buffer, byteOffset, length);
}
function TypedArrayEntries() {
    var O = this;
    IsTypedArrayEnsuringArrayBuffer(O);
    return CreateArrayIterator(O, 2);
}
function TypedArrayEvery(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.every");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var thisArg = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        var kValue = O[k];
        var testResult = callContentFunction(callbackfn, thisArg, kValue, k, O);
        if (!testResult)
            return false;
    }
    return true;
}
function TypedArrayFill(value, start = 0, end = undefined) {
    if (!IsObject(this) || !IsTypedArray(this)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, value, start, end,
                            "TypedArrayFill");
    }
    var O = this;
    var buffer = GetAttachedArrayBuffer(this);
    var len = TypedArrayLength(O);
    var kind = GetTypedArrayKind(O);
    if (kind === 9 || kind === 10) {
        value = ToBigInt(value);
    } else {
        value = ToNumber(value);
    }
    var relativeStart = ToInteger(start);
    var k = relativeStart < 0
            ? std_Math_max(len + relativeStart, 0)
            : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0
                ? std_Math_max(len + relativeEnd, 0)
                : std_Math_min(relativeEnd, len);
    if (buffer === null) {
        buffer = ViewedArrayBufferIfReified(O);
    }
    if (IsDetachedBuffer(buffer))
        ThrowTypeError(530);
    for (; k < final; k++) {
        O[k] = value;
    }
    return O;
}
function TypedArrayFilter(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.filter");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    var kept = new List();
    var captured = 0;
    for (var k = 0; k < len; k++) {
        var kValue = O[k];
        var selected = ToBoolean(callContentFunction(callbackfn, T, kValue, k, O));
        if (selected) {
            kept[captured++] = kValue;
        }
    }
    var A = TypedArraySpeciesCreateWithLength(O, captured);
    for (var n = 0; n < captured; n++) {
        A[n] = kept[n];
    }
    return A;
}
function TypedArrayFind(predicate ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.find");
    if (!IsCallable(predicate))
        ThrowTypeError(10, DecompileArg(0, predicate));
    var thisArg = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        var kValue = O[k];
        if (callContentFunction(predicate, thisArg, kValue, k, O))
            return kValue;
    }
    return undefined;
}
function TypedArrayFindIndex(predicate ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.findIndex");
    if (!IsCallable(predicate))
        ThrowTypeError(10, DecompileArg(0, predicate));
    var thisArg = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        if (callContentFunction(predicate, thisArg, O[k], k, O))
            return k;
    }
    return -1;
}
function TypedArrayForEach(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "TypedArray.prototype.forEach");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var thisArg = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        callContentFunction(callbackfn, thisArg, O[k], k, O);
    }
    return undefined;
}
function TypedArrayIndexOf(searchElement, fromIndex = 0) {
    if (!IsObject(this) || !IsTypedArray(this)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, searchElement, fromIndex,
                            "TypedArrayIndexOf");
    }
    GetAttachedArrayBuffer(this);
    var O = this;
    var len = TypedArrayLength(O);
    if (len === 0)
        return -1;
    var n = ToInteger(fromIndex);
    ;;
    len = TypedArrayLength(O);
    ;;
    if (n >= len)
        return -1;
    var k;
    if (n >= 0) {
        k = n;
    } else {
        k = len + n;
        if (k < 0)
            k = 0;
    }
    for (; k < len; k++) {
        ;;
        if (O[k] === searchElement)
            return k;
    }
    return -1;
}
function TypedArrayJoin(separator) {
    if (!IsObject(this) || !IsTypedArray(this)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, separator, "TypedArrayJoin");
    }
    GetAttachedArrayBuffer(this);
    var O = this;
    var len = TypedArrayLength(O);
    var sep = separator === undefined ? "," : ToString(separator);
    if (len === 0)
        return "";
    if (TypedArrayLength(O) === 0) {
        ;;
        return callFunction(String_repeat, ",", len - 1);
    }
    ;;
    var element0 = O[0];
    ;;
    var R = ToString(element0);
    for (var k = 1; k < len; k++) {
        var element = O[k];
        ;;
        R += sep + ToString(element);
    }
    return R;
}
function TypedArrayKeys() {
    var O = this;
    IsTypedArrayEnsuringArrayBuffer(O);
    return CreateArrayIterator(O, 0);
}
function TypedArrayLastIndexOf(searchElement ) {
    if (!IsObject(this) || !IsTypedArray(this)) {
        if (arguments.length > 1) {
            return callFunction(CallTypedArrayMethodIfWrapped, this, searchElement, arguments[1],
                                "TypedArrayLastIndexOf");
        }
        return callFunction(CallTypedArrayMethodIfWrapped, this, searchElement,
                            "TypedArrayLastIndexOf");
    }
    GetAttachedArrayBuffer(this);
    var O = this;
    var len = TypedArrayLength(O);
    if (len === 0)
        return -1;
    var n = arguments.length > 1 ? ToInteger(arguments[1]) : len - 1;
    len = TypedArrayLength(O);
    ;;
    var k = n >= 0 ? std_Math_min(n, len - 1) : len + n;
    for (; k >= 0; k--) {
        ;;
        if (O[k] === searchElement)
            return k;
    }
    return -1;
}
function TypedArrayMap(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.map");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var T = arguments.length > 1 ? arguments[1] : void 0;
    var A = TypedArraySpeciesCreateWithLength(O, len);
    for (var k = 0; k < len; k++) {
        var mappedValue = callContentFunction(callbackfn, T, O[k], k, O);
        A[k] = mappedValue;
    }
    return A;
}
function TypedArrayReduce(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.reduce");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    if (len === 0 && arguments.length === 1)
        ThrowTypeError(44);
    var k = 0;
    var accumulator = arguments.length > 1 ? arguments[1] : O[k++];
    for (; k < len; k++) {
        accumulator = callContentFunction(callbackfn, undefined, accumulator, O[k], k, O);
    }
    return accumulator;
}
function TypedArrayReduceRight(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.reduceRight");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    if (len === 0 && arguments.length === 1)
        ThrowTypeError(44);
    var k = len - 1;
    var accumulator = arguments.length > 1 ? arguments[1] : O[k--];
    for (; k >= 0; k--) {
        accumulator = callContentFunction(callbackfn, undefined, accumulator, O[k], k, O);
    }
    return accumulator;
}
function TypedArrayReverse() {
    if (!IsObject(this) || !IsTypedArray(this)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, "TypedArrayReverse");
    }
    GetAttachedArrayBuffer(this);
    var O = this;
    var len = TypedArrayLength(O);
    var middle = std_Math_floor(len / 2);
    for (var lower = 0; lower !== middle; lower++) {
        var upper = len - lower - 1;
        var lowerValue = O[lower];
        var upperValue = O[upper];
        O[lower] = upperValue;
        O[upper] = lowerValue;
    }
    return O;
}
function TypedArraySlice(start, end) {
    var O = this;
    if (!IsObject(O) || !IsTypedArray(O)) {
        return callFunction(CallTypedArrayMethodIfWrapped, O, start, end, "TypedArraySlice");
    }
    var buffer = GetAttachedArrayBuffer(O);
    var len = TypedArrayLength(O);
    var relativeStart = ToInteger(start);
    var k = relativeStart < 0
            ? std_Math_max(len + relativeStart, 0)
            : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0
                ? std_Math_max(len + relativeEnd, 0)
                : std_Math_min(relativeEnd, len);
    var count = std_Math_max(final - k, 0);
    var A = TypedArraySpeciesCreateWithLength(O, count);
    if (count > 0) {
        if (buffer === null) {
            buffer = ViewedArrayBufferIfReified(O);
        }
        if (IsDetachedBuffer(buffer))
            ThrowTypeError(530);
        var sliced = TypedArrayBitwiseSlice(O, A, k, count);
        if (!sliced) {
            var n = 0;
            while (k < final) {
                A[n++] = O[k++];
            }
        }
    }
    return A;
}
function TypedArraySome(callbackfn ) {
    var O = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(O);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(O);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, O, "TypedArrayLengthMethod");
    if (arguments.length === 0)
        ThrowTypeError(47, 0, "%TypedArray%.prototype.some");
    if (!IsCallable(callbackfn))
        ThrowTypeError(10, DecompileArg(0, callbackfn));
    var thisArg = arguments.length > 1 ? arguments[1] : void 0;
    for (var k = 0; k < len; k++) {
        var kValue = O[k];
        var testResult = callContentFunction(callbackfn, thisArg, kValue, k, O);
        if (testResult)
            return true;
    }
    return false;
}
function TypedArrayCompare(x, y) {
    ;;
    if (x < y)
        return -1;
    if (x > y)
        return 1;
    if (x === 0 && y === 0)
        return ((1 / x) > 0 ? 1 : 0) - ((1 / y) > 0 ? 1 : 0);
    if (Number_isNaN(x))
        return Number_isNaN(y) ? 0 : 1;
    return Number_isNaN(y) ? -1 : 0;
}
function TypedArrayCompareInt(x, y) {
    ;;
    ;;
    var diff = x - y;
    if (diff)
        return diff;
    return 0;
}
function TypedArrayCompareBigInt(x, y) {
    ;;
    if (x < y)
        return -1;
    if (x > y)
        return 1;
    return 0;
}
function TypedArraySort(comparefn) {
    if (comparefn !== undefined) {
        if (!IsCallable(comparefn))
            ThrowTypeError(10, DecompileArg(0, comparefn));
    }
    var obj = this;
    var isTypedArray = IsObject(obj) && IsTypedArray(obj);
    var buffer;
    if (isTypedArray) {
        buffer = GetAttachedArrayBuffer(obj);
    } else {
        buffer = callFunction(CallTypedArrayMethodIfWrapped, obj, "GetAttachedArrayBufferMethod");
    }
    var len;
    if (isTypedArray) {
        len = TypedArrayLength(obj);
    } else {
        len = callFunction(CallTypedArrayMethodIfWrapped, obj, "TypedArrayLengthMethod");
    }
    if (len <= 1)
        return obj;
    if (comparefn === undefined) {
        var kind = GetTypedArrayKind(obj);
        switch (kind) {
          case 1:
          case 8:
            return CountingSort(obj, len, false , TypedArrayCompareInt);
          case 0:
            return CountingSort(obj, len, true , TypedArrayCompareInt);
          case 3:
            return RadixSort(obj, len, buffer,
                             2 , false , false ,
                             TypedArrayCompareInt);
          case 2:
            return RadixSort(obj, len, buffer,
                             2 , true , false ,
                             TypedArrayCompareInt);
          case 5:
            return RadixSort(obj, len, buffer,
                             4 , false , false ,
                             TypedArrayCompareInt);
          case 4:
            return RadixSort(obj, len, buffer,
                             4 , true , false ,
                             TypedArrayCompareInt);
          case 9:
          case 10:
            return QuickSort(obj, len, TypedArrayCompareBigInt);
          case 6:
            return RadixSort(obj, len, buffer,
                             4 , true , true ,
                             TypedArrayCompare);
          case 7:
          default:
            ;;
            return QuickSort(obj, len, TypedArrayCompare);
        }
    }
    var wrappedCompareFn = function(x, y) {
        var v = +comparefn(x, y);
        var length;
        if (isTypedArray) {
            length = TypedArrayLength(obj);
        } else {
            length = callFunction(CallTypedArrayMethodIfWrapped, obj, "TypedArrayLengthMethod");
        }
        if (length === 0) {
            ;;
            ThrowTypeError(530);
        }
        if (v !== v)
            return 0;
        return v;
    };
    return MergeSortTypedArray(obj, len, wrappedCompareFn);
}
function TypedArrayToLocaleString(locales = undefined, options = undefined) {
    var array = this;
    var isTypedArray = IsTypedArrayEnsuringArrayBuffer(array);
    var len;
    if (isTypedArray)
        len = TypedArrayLength(array);
    else
        len = callFunction(CallTypedArrayMethodIfWrapped, array, "TypedArrayLengthMethod");
    if (len === 0)
        return "";
    var firstElement = array[0];
    var R = ToString(callContentFunction(firstElement.toLocaleString, firstElement, locales, options));
    var separator = ",";
    for (var k = 1; k < len; k++) {
        var S = R + separator;
        var nextElement = array[k];
        R = ToString(callContentFunction(nextElement.toLocaleString, nextElement, locales, options));
        R = S + R;
    }
    return R;
}
function TypedArraySubarray(begin, end) {
    var obj = this;
    if (!IsObject(obj) || !IsTypedArray(obj)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, begin, end,
                            "TypedArraySubarray");
    }
    var buffer = ViewedArrayBufferIfReified(obj);
    if (buffer === null) {
        buffer = TypedArrayBuffer(obj);
    }
    var srcLength = TypedArrayLength(obj);
    var srcByteOffset = TypedArrayByteOffset(obj);
    var relativeBegin = ToInteger(begin);
    var beginIndex = relativeBegin < 0 ? std_Math_max(srcLength + relativeBegin, 0)
                                       : std_Math_min(relativeBegin, srcLength);
    var relativeEnd = end === undefined ? srcLength : ToInteger(end);
    var endIndex = relativeEnd < 0 ? std_Math_max(srcLength + relativeEnd, 0)
                                   : std_Math_min(relativeEnd, srcLength);
    var newLength = std_Math_max(endIndex - beginIndex, 0);
    var elementSize = TypedArrayElementSize(obj);
    var beginByteOffset = srcByteOffset + (beginIndex * elementSize);
    return TypedArraySpeciesCreateWithBuffer(obj, buffer, beginByteOffset, newLength);
}
function TypedArrayAt(index) {
    var obj = this;
    if (!IsObject(obj) || !IsTypedArray(obj)) {
        return callFunction(CallTypedArrayMethodIfWrapped, obj, index,
                            "TypedArrayAt");
    }
    GetAttachedArrayBuffer(obj);
    var len = TypedArrayLength(obj);
    var relativeIndex = ToInteger(index);
    var k;
    if (relativeIndex >= 0) {
        k = relativeIndex;
    } else {
        k = len + relativeIndex;
    }
    if (k < 0 || k >= len) {
        return undefined;
    }
    return obj[k];
}
function $TypedArrayValues() {
    var O = this;
    IsTypedArrayEnsuringArrayBuffer(O);
    return CreateArrayIterator(O, 1);
}
_SetCanonicalName($TypedArrayValues, "values");
function TypedArrayIncludes(searchElement, fromIndex = 0) {
    if (!IsObject(this) || !IsTypedArray(this)) {
        return callFunction(CallTypedArrayMethodIfWrapped, this, searchElement,
                            fromIndex, "TypedArrayIncludes");
    }
    GetAttachedArrayBuffer(this);
    var O = this;
    var len = TypedArrayLength(O);
    if (len === 0)
        return false;
    var n = ToInteger(fromIndex);
    ;;
    var k;
    if (n >= 0) {
        k = n;
    } else {
        k = len + n;
        if (k < 0)
            k = 0;
    }
    while (k < len) {
        if (SameValueZero(searchElement, O[k]))
            return true;
        k++;
    }
    return false;
}
function TypedArrayStaticFrom(source, mapfn = undefined, thisArg = undefined) {
    var C = this;
    if (!IsConstructor(C))
        ThrowTypeError(11, typeof C);
    var mapping;
    if (mapfn !== undefined) {
        if (!IsCallable(mapfn))
            ThrowTypeError(10, DecompileArg(1, mapfn));
        mapping = true;
    } else {
        mapping = false;
    }
    var T = thisArg;
    var usingIterator = source[std_iterator];
    if (usingIterator !== undefined && usingIterator !== null) {
        if (!IsCallable(usingIterator))
            ThrowTypeError(65, DecompileArg(0, source));
        if (!mapping && IsTypedArrayConstructor(C) && IsObject(source)) {
            if (usingIterator === $TypedArrayValues && IsTypedArray(source) &&
                ArrayIteratorPrototypeOptimizable())
            {
                GetAttachedArrayBuffer(source);
                var len = TypedArrayLength(source);
                var targetObj = new C(len);
                for (var k = 0; k < len; k++) {
                    targetObj[k] = source[k];
                }
                return targetObj;
            }
            if (usingIterator === $ArrayValues && IsPackedArray(source) &&
                ArrayIteratorPrototypeOptimizable())
            {
                var targetObj = new C(source.length);
                TypedArrayInitFromPackedArray(targetObj, source);
                return targetObj;
            }
        }
        var values = IterableToList(source, usingIterator);
        var len = values.length;
        var targetObj = TypedArrayCreateWithLength(C, len);
        for (var k = 0; k < len; k++) {
            var kValue = values[k];
            var mappedValue = mapping ? callContentFunction(mapfn, T, kValue, k) : kValue;
            targetObj[k] = mappedValue;
        }
        return targetObj;
    }
    var arrayLike = ToObject(source);
    var len = ToLength(arrayLike.length);
    var targetObj = TypedArrayCreateWithLength(C, len);
    for (var k = 0; k < len; k++) {
        var kValue = arrayLike[k];
        var mappedValue = mapping ? callContentFunction(mapfn, T, kValue, k) : kValue;
        targetObj[k] = mappedValue;
    }
    return targetObj;
}
function TypedArrayStaticOf( ) {
    var len = arguments.length;
    var items = arguments;
    var C = this;
    if (!IsConstructor(C))
        ThrowTypeError(11, typeof C);
    var newObj = TypedArrayCreateWithLength(C, len);
    for (var k = 0; k < len; k++)
        newObj[k] = items[k];
    return newObj;
}
function $TypedArraySpecies() {
    return this;
}
_SetCanonicalName($TypedArraySpecies, "get [Symbol.species]");
function IterableToList(items, method) {
    ;;
    var iterator = callContentFunction(method, items);
    if (!IsObject(iterator))
        ThrowTypeError(67);
    var nextMethod = iterator.next;
    var values = [];
    var i = 0;
    while (true) {
        var next = callContentFunction(nextMethod, iterator);
        if (!IsObject(next))
            ThrowTypeError(68, "next");
        if (next.done)
            break;
        _DefineDataProperty(values, i++, next.value);
    }
    return values;
}
function ArrayBufferSlice(start, end) {
    var O = this;
    if (!IsObject(O) || (O = GuardToArrayBuffer(O)) === null) {
        return callFunction(CallArrayBufferMethodIfWrapped, this, start, end,
                            "ArrayBufferSlice");
    }
    if (IsDetachedBuffer(O))
        ThrowTypeError(530);
    var len = ArrayBufferByteLength(O);
    var relativeStart = ToInteger(start);
    var first = relativeStart < 0 ? std_Math_max(len + relativeStart, 0)
                                  : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0 ? std_Math_max(len + relativeEnd, 0)
                                : std_Math_min(relativeEnd, len);
    var newLen = std_Math_max(final - first, 0);
    var ctor = SpeciesConstructor(O, GetBuiltinConstructor("ArrayBuffer"));
    var new_ = new ctor(newLen);
    var isWrapped = false;
    var newBuffer;
    if ((newBuffer = GuardToArrayBuffer(new_)) !== null) {
        if (IsDetachedBuffer(newBuffer))
            ThrowTypeError(530);
    } else {
        newBuffer = new_;
        if (!IsWrappedArrayBuffer(newBuffer))
            ThrowTypeError(525);
        isWrapped = true;
        if (callFunction(CallArrayBufferMethodIfWrapped, newBuffer, "IsDetachedBufferThis"))
            ThrowTypeError(530);
    }
    if (newBuffer === O)
        ThrowTypeError(526);
    var actualLen = PossiblyWrappedArrayBufferByteLength(newBuffer);
    if (actualLen < newLen)
        ThrowTypeError(527, newLen, actualLen);
    if (IsDetachedBuffer(O))
        ThrowTypeError(530);
    ArrayBufferCopyData(newBuffer, 0, O, first, newLen, isWrapped);
    return newBuffer;
}
function IsDetachedBufferThis() {
  return IsDetachedBuffer(this);
}
function $ArrayBufferSpecies() {
    return this;
}
_SetCanonicalName($ArrayBufferSpecies, "get [Symbol.species]");
function $SharedArrayBufferSpecies() {
    return this;
}
_SetCanonicalName($SharedArrayBufferSpecies, "get [Symbol.species]");
function SharedArrayBufferSlice(start, end) {
    var O = this;
    if (!IsObject(O) || (O = GuardToSharedArrayBuffer(O)) === null) {
        return callFunction(CallSharedArrayBufferMethodIfWrapped, this, start, end,
                            "SharedArrayBufferSlice");
    }
    var len = SharedArrayBufferByteLength(O);
    var relativeStart = ToInteger(start);
    var first = relativeStart < 0 ? std_Math_max(len + relativeStart, 0)
                                  : std_Math_min(relativeStart, len);
    var relativeEnd = end === undefined ? len : ToInteger(end);
    var final = relativeEnd < 0 ? std_Math_max(len + relativeEnd, 0)
                                : std_Math_min(relativeEnd, len);
    var newLen = std_Math_max(final - first, 0);
    var ctor = SpeciesConstructor(O, GetBuiltinConstructor("SharedArrayBuffer"));
    var new_ = new ctor(newLen);
    var isWrapped = false;
    var newObj;
    if ((newObj = GuardToSharedArrayBuffer(new_)) === null) {
        if (!IsWrappedSharedArrayBuffer(new_))
            ThrowTypeError(538);
        isWrapped = true;
        newObj = new_;
    }
    if (newObj === O || SharedArrayBuffersMemorySame(newObj, O))
        ThrowTypeError(539);
    var actualLen = PossiblyWrappedSharedArrayBufferByteLength(newObj);
    if (actualLen < newLen)
        ThrowTypeError(540, newLen, actualLen);
    SharedArrayBufferCopyData(newObj, 0, O, first, newLen, isWrapped);
    return newObj;
}
function WeakMapConstructorInit(iterable) {
    var map = this;
    var adder = map.set;
    if (!IsCallable(adder))
        ThrowTypeError(10, typeof adder);
    for (var nextItem of allowContentIter(iterable)) {
        if (!IsObject(nextItem))
            ThrowTypeError(37, "WeakMap");
        callContentFunction(adder, map, nextItem[0], nextItem[1]);
    }
}
function WeakSetConstructorInit(iterable) {
    var set = this;
    var adder = set.add;
    if (!IsCallable(adder))
        ThrowTypeError(10, typeof adder);
    for (var nextValue of allowContentIter(iterable))
        callContentFunction(adder, set, nextValue);
}
function resolveCollatorInternals(lazyCollatorData) {
    ;;
    var internalProps = std_Object_create(null);
    var Collator = collatorInternalProperties;
    internalProps.usage = lazyCollatorData.usage;
    var collatorIsSorting = lazyCollatorData.usage === "sort";
    var localeData = collatorIsSorting
                     ? Collator.sortLocaleData
                     : Collator.searchLocaleData;
    var relevantExtensionKeys = Collator.relevantExtensionKeys;
    var r = ResolveLocale("Collator",
                          lazyCollatorData.requestedLocales,
                          lazyCollatorData.opt,
                          relevantExtensionKeys,
                          localeData);
    internalProps.locale = r.locale;
    var collation = r.co;
    if (collation === null)
        collation = "default";
    internalProps.collation = collation;
    internalProps.numeric = r.kn === "true";
    internalProps.caseFirst = r.kf;
    var s = lazyCollatorData.rawSensitivity;
    if (s === undefined) {
        s = "variant";
    }
    internalProps.sensitivity = s;
    internalProps.ignorePunctuation = lazyCollatorData.ignorePunctuation;
    return internalProps;
}
function getCollatorInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveCollatorInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function InitializeCollator(collator, locales, options) {
    ;;
    ;;
    var lazyCollatorData = std_Object_create(null);
    var requestedLocales = CanonicalizeLocaleList(locales);
    lazyCollatorData.requestedLocales = requestedLocales;
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    var u = GetOption(options, "usage", "string", ["sort", "search"], "sort");
    lazyCollatorData.usage = u;
    var opt = new Record();
    lazyCollatorData.opt = opt;
    var matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    var collation = GetOption(options, "collation", "string", undefined, undefined);
    if (collation !== undefined)
        collation = intl_ValidateAndCanonicalizeUnicodeExtensionType(collation, "collation", "co");
    opt.co = collation;
    var numericValue = GetOption(options, "numeric", "boolean", undefined, undefined);
    if (numericValue !== undefined)
        numericValue = numericValue ? "true" : "false";
    opt.kn = numericValue;
    var caseFirstValue = GetOption(options, "caseFirst", "string", ["upper", "lower", "false"], undefined);
    opt.kf = caseFirstValue;
    var s = GetOption(options, "sensitivity", "string",
                      ["base", "accent", "case", "variant"], undefined);
    lazyCollatorData.rawSensitivity = s;
    var ip = GetOption(options, "ignorePunctuation", "boolean", undefined, false);
    lazyCollatorData.ignorePunctuation = ip;
    initializeIntlObject(collator, "Collator", lazyCollatorData);
}
function Intl_Collator_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "Collator";
    var requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
var collatorInternalProperties = {
    sortLocaleData: collatorSortLocaleData,
    searchLocaleData: collatorSearchLocaleData,
    relevantExtensionKeys: ["co", "kf", "kn"],
};
function collatorActualLocale(locale) {
    ;;
    return BestAvailableLocaleIgnoringDefault("Collator", locale);
}
function collatorSortCaseFirst(locale) {
    var actualLocale = collatorActualLocale(locale);
    if (intl_isUpperCaseFirst(actualLocale))
        return ["upper", "false", "lower"];
    return ["false", "lower", "upper"];
}
function collatorSortCaseFirstDefault(locale) {
    var actualLocale = collatorActualLocale(locale);
    if (intl_isUpperCaseFirst(actualLocale))
        return "upper";
    return "false";
}
function collatorSortLocaleData() {
    return {
        co: intl_availableCollations,
        kn: function() {
            return ["false", "true"];
        },
        kf: collatorSortCaseFirst,
        default: {
            co: function() {
                return null;
            },
            kn: function() {
                return "false";
            },
            kf: collatorSortCaseFirstDefault,
        },
    };
}
function collatorSearchLocaleData() {
    return {
        co: function() {
            return [null];
        },
        kn: function() {
            return ["false", "true"];
        },
        kf: function() {
            return ["false", "lower", "upper"];
        },
        default: {
            co: function() {
                return null;
            },
            kn: function() {
                return "false";
            },
            kf: function() {
                return "false";
            },
        },
    };
}
function createCollatorCompare(collator) {
    return function(x, y) {
        ;;
        ;;
        var X = ToString(x);
        var Y = ToString(y);
        return intl_CompareStrings(collator, X, Y);
    };
}
function $Intl_Collator_compare_get() {
    var collator = this;
    if (!IsObject(collator) || (collator = GuardToCollator(collator)) === null)
        return callFunction(CallCollatorMethodIfWrapped, this, "$Intl_Collator_compare_get");
    var internals = getCollatorInternals(collator);
    if (internals.boundCompare === undefined) {
        internals.boundCompare = createCollatorCompare(collator);
    }
    return internals.boundCompare;
}
_SetCanonicalName($Intl_Collator_compare_get, "get compare");
function Intl_Collator_resolvedOptions() {
    var collator = this;
    if (!IsObject(collator) || (collator = GuardToCollator(collator)) === null)
        return callFunction(CallCollatorMethodIfWrapped, this, "Intl_Collator_resolvedOptions");
    var internals = getCollatorInternals(collator);
    var result = {
        locale: internals.locale,
        usage: internals.usage,
        sensitivity: internals.sensitivity,
        ignorePunctuation: internals.ignorePunctuation,
        collation: internals.collation,
        numeric: internals.numeric,
        caseFirst: internals.caseFirst,
    };
    return result;
}
function startOfUnicodeExtensions(locale) {
    ;;
    var start = callFunction(std_String_indexOf, locale, "-u-");
    if (start < 0)
        return -1;
    var privateExt = callFunction(std_String_indexOf, locale, "-x-");
    if (privateExt >= 0 && privateExt < start)
        return -1;
    return start;
}
function endOfUnicodeExtensions(locale, start) {
    ;;
    ;;
    ;;
    ;;
    for (var i = start + 5, end = locale.length - 4; i <= end; i++) {
        if (callFunction(std_String_charCodeAt, locale, i) !== 0x2D)
            continue;
        if (callFunction(std_String_charCodeAt, locale, i + 2) === 0x2D)
            return i;
        i += 2;
    }
    return locale.length;
}
function removeUnicodeExtensions(locale) {
    ;;
    var start = startOfUnicodeExtensions(locale);
    if (start < 0)
        return locale;
    var end = endOfUnicodeExtensions(locale, start);
    var left = Substring(locale, 0, start);
    var right = Substring(locale, end, locale.length - end);
    var combined = left + right;
    ;;
    ;;
    return combined;
}
function getUnicodeExtensions(locale) {
    ;;
    var start = startOfUnicodeExtensions(locale);
    ;;
    var end = endOfUnicodeExtensions(locale, start);
    return Substring(locale, start, end - start);
}
function IsASCIIAlphaString(s) {
    ;;
    for (var i = 0; i < s.length; i++) {
        var c = callFunction(std_String_charCodeAt, s, i);
        if (!((0x41 <= c && c <= 0x5A) || (0x61 <= c && c <= 0x7A)))
            return false;
    }
    return true;
}
var localeCache = {
    runtimeDefaultLocale: undefined,
    defaultLocale: undefined,
};
function DefaultLocale() {
    if (IsRuntimeDefaultLocale(localeCache.runtimeDefaultLocale))
        return localeCache.defaultLocale;
    var runtimeDefaultLocale = RuntimeDefaultLocale();
    var locale = intl_supportedLocaleOrFallback(runtimeDefaultLocale);
    ;;
    ;;
    localeCache.defaultLocale = locale;
    localeCache.runtimeDefaultLocale = runtimeDefaultLocale;
    return locale;
}
function CanonicalizeLocaleList(locales) {
    if (locales === undefined)
        return [];
    var tag = intl_ValidateAndCanonicalizeLanguageTag(locales, false);
    if (tag !== null) {
        ;;
        return [tag];
    }
    var seen = [];
    var O = ToObject(locales);
    var len = ToLength(O.length);
    var k = 0;
    while (k < len) {
        if (k in O) {
            var kValue = O[k];
            if (!(typeof kValue === "string" || IsObject(kValue)))
                ThrowTypeError(482);
            var tag = intl_ValidateAndCanonicalizeLanguageTag(kValue, true);
            ;;
            if (callFunction(ArrayIndexOf, seen, tag) === -1)
                _DefineDataProperty(seen, seen.length, tag);
        }
        k++;
    }
    return seen;
}
function BestAvailableLocale(availableLocales, locale) {
    return intl_BestAvailableLocale(availableLocales, locale, DefaultLocale());
}
function BestAvailableLocaleIgnoringDefault(availableLocales, locale) {
    return intl_BestAvailableLocale(availableLocales, locale, null);
}
function LookupMatcher(availableLocales, requestedLocales) {
    var result = new Record();
    for (var i = 0; i < requestedLocales.length; i++) {
        var locale = requestedLocales[i];
        var noExtensionsLocale = removeUnicodeExtensions(locale);
        var availableLocale = BestAvailableLocale(availableLocales, noExtensionsLocale);
        if (availableLocale !== undefined) {
            result.locale = availableLocale;
            if (locale !== noExtensionsLocale)
                result.extension = getUnicodeExtensions(locale);
            return result;
        }
    }
    result.locale = DefaultLocale();
    return result;
}
function BestFitMatcher(availableLocales, requestedLocales) {
    return LookupMatcher(availableLocales, requestedLocales);
}
function UnicodeExtensionValue(extension, key) {
    ;;
    ;;
    ;;
    ;;
    var size = extension.length;
    var searchValue = "-" + key + "-";
    var pos = callFunction(std_String_indexOf, extension, searchValue);
    if (pos !== -1) {
        var start = pos + 4;
        var end = start;
        var k = start;
        while (true) {
            var e = callFunction(std_String_indexOf, extension, "-", k);
            var len = e === -1 ? size - k : e - k;
            if (len === 2)
                break;
            if (e === -1) {
                end = size;
                break;
            }
            end = e;
            k = e + 1;
        }
        return callFunction(String_substring, extension, start, end);
    }
    searchValue = "-" + key;
    if (callFunction(std_String_endsWith, extension, searchValue))
        return "";
}
function ResolveLocale(availableLocales, requestedLocales, options, relevantExtensionKeys, localeData) {
    var matcher = options.localeMatcher;
    var r = (matcher === "lookup")
            ? LookupMatcher(availableLocales, requestedLocales)
            : BestFitMatcher(availableLocales, requestedLocales);
    var foundLocale = r.locale;
    var extension = r.extension;
    var result = new Record();
    result.dataLocale = foundLocale;
    var supportedExtension = "-u";
    var localeDataProvider = localeData();
    for (var i = 0; i < relevantExtensionKeys.length; i++) {
        var key = relevantExtensionKeys[i];
        var keyLocaleData = undefined;
        var value = undefined;
        var supportedExtensionAddition = "";
        if (extension !== undefined) {
            var requestedValue = UnicodeExtensionValue(extension, key);
            if (requestedValue !== undefined) {
                keyLocaleData = callFunction(localeDataProvider[key], null, foundLocale);
                if (requestedValue !== "") {
                    if (callFunction(ArrayIndexOf, keyLocaleData, requestedValue) !== -1) {
                        value = requestedValue;
                        supportedExtensionAddition = "-" + key + "-" + value;
                    }
                } else {
                    if (callFunction(ArrayIndexOf, keyLocaleData, "true") !== -1) {
                        value = "true";
                        supportedExtensionAddition = "-" + key;
                    }
                }
            }
        }
        var optionsValue = options[key];
        ;;
        if (optionsValue !== undefined && optionsValue !== value) {
            if (keyLocaleData === undefined)
                keyLocaleData = callFunction(localeDataProvider[key], null, foundLocale);
            if (callFunction(ArrayIndexOf, keyLocaleData, optionsValue) !== -1) {
                value = optionsValue;
                supportedExtensionAddition = "";
            }
        }
        if (value === undefined) {
            value = keyLocaleData === undefined
                    ? callFunction(localeDataProvider.default[key], null, foundLocale)
                    : keyLocaleData[0];
        }
        ;;
        result[key] = value;
        supportedExtension += supportedExtensionAddition;
    }
    if (supportedExtension.length > 2)
        foundLocale = addUnicodeExtension(foundLocale, supportedExtension);
    result.locale = foundLocale;
    return result;
}
function addUnicodeExtension(locale, extension) {
    ;;
    ;;
    ;;
    ;;
    ;;
    var privateIndex = callFunction(std_String_indexOf, locale, "-x-");
    if (privateIndex === -1) {
        locale += extension;
    } else {
         var preExtension = callFunction(String_substring, locale, 0, privateIndex);
         var postExtension = callFunction(String_substring, locale, privateIndex);
         locale = preExtension + extension + postExtension;
    }
    ;;
    return locale;
}
function LookupSupportedLocales(availableLocales, requestedLocales) {
    var subset = [];
    for (var i = 0; i < requestedLocales.length; i++) {
        var locale = requestedLocales[i];
        var noExtensionsLocale = removeUnicodeExtensions(locale);
        var availableLocale = BestAvailableLocale(availableLocales, noExtensionsLocale);
        if (availableLocale !== undefined)
            _DefineDataProperty(subset, subset.length, locale);
    }
    return subset;
}
function BestFitSupportedLocales(availableLocales, requestedLocales) {
    return LookupSupportedLocales(availableLocales, requestedLocales);
}
function SupportedLocales(availableLocales, requestedLocales, options) {
    var matcher;
    if (options !== undefined) {
        options = ToObject(options);
        matcher = options.localeMatcher;
        if (matcher !== undefined) {
            matcher = ToString(matcher);
            if (matcher !== "lookup" && matcher !== "best fit")
                ThrowRangeError(483, matcher);
        }
    }
    return (matcher === undefined || matcher === "best fit")
           ? BestFitSupportedLocales(availableLocales, requestedLocales)
           : LookupSupportedLocales(availableLocales, requestedLocales);
}
function GetOption(options, property, type, values, fallback) {
    var value = options[property];
    if (value !== undefined) {
        if (type === "boolean")
            value = ToBoolean(value);
        else if (type === "string")
            value = ToString(value);
        else
            ;;
        if (values !== undefined && callFunction(ArrayIndexOf, values, value) === -1)
            ThrowRangeError(484, property, `"${value}"`);
        return value;
    }
    return fallback;
}
function DefaultNumberOption(value, minimum, maximum, fallback) {
    ;;
    ;;
    ;;
    ;;
    if (value === undefined)
        return fallback;
    value = ToNumber(value);
    if (Number_isNaN(value) || value < minimum || value > maximum)
        ThrowRangeError(478, value);
    return std_Math_floor(value) | 0;
}
function GetNumberOption(options, property, minimum, maximum, fallback) {
    return DefaultNumberOption(options[property], minimum, maximum, fallback);
}
var intlFallbackSymbolHolder = { value: undefined };
function intlFallbackSymbol() {
    var fallbackSymbol = intlFallbackSymbolHolder.value;
    if (!fallbackSymbol) {
        fallbackSymbol = std_Symbol("IntlLegacyConstructedSymbol");
        intlFallbackSymbolHolder.value = fallbackSymbol;
    }
    return fallbackSymbol;
}
function initializeIntlObject(obj, type, lazyData) {
    ;;
    ;;
    ;;
    var internals = std_Object_create(null);
    internals.type = type;
    internals.lazyData = lazyData;
    internals.internalProps = null;
    ;;
    UnsafeSetReservedSlot(obj, 0, internals);
}
function setInternalProperties(internals, internalProps) {
    ;;
    ;;
    internals.internalProps = internalProps;
    internals.lazyData = null;
}
function maybeInternalProperties(internals) {
    ;;
    var lazyData = internals.lazyData;
    if (lazyData)
        return null;
    ;;
    return internals.internalProps;
}
function getIntlObjectInternals(obj) {
    ;;
    ;;
    var internals = UnsafeGetReservedSlot(obj, 0);
    ;;
    ;;
    ;;
    ;;
    ;;
    return internals;
}
function getInternals(obj) {
    var internals = getIntlObjectInternals(obj);
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    var type = internals.type;
    if (type === "Collator")
        internalProps = resolveCollatorInternals(internals.lazyData);
    else if (type === "DateTimeFormat")
        internalProps = resolveDateTimeFormatInternals(internals.lazyData);
    else if (type === "DisplayNames")
      internalProps = resolveDisplayNamesInternals(internals.lazyData);
    else if (type === "ListFormat")
        internalProps = resolveListFormatInternals(internals.lazyData);
    else if (type === "NumberFormat")
        internalProps = resolveNumberFormatInternals(internals.lazyData);
    else if (type === "PluralRules")
        internalProps = resolvePluralRulesInternals(internals.lazyData);
    else
        internalProps = resolveRelativeTimeFormatInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
var currencyDigits = {
    BHD: 3,
    BIF: 0,
    CLF: 4,
    CLP: 0,
    DJF: 0,
    GNF: 0,
    IQD: 3,
    ISK: 0,
    JOD: 3,
    JPY: 0,
    KMF: 0,
    KRW: 0,
    KWD: 3,
    LYD: 3,
    OMR: 3,
    PYG: 0,
    RWF: 0,
    TND: 3,
    UGX: 0,
    UYI: 0,
    UYW: 4,
    VND: 0,
    VUV: 0,
    XAF: 0,
    XOF: 0,
    XPF: 0,
};
function resolveDateTimeFormatInternals(lazyDateTimeFormatData) {
    ;;
    var internalProps = std_Object_create(null);
    var DateTimeFormat = dateTimeFormatInternalProperties;
    var localeData = DateTimeFormat.localeData;
    var r = ResolveLocale("DateTimeFormat",
                          lazyDateTimeFormatData.requestedLocales,
                          lazyDateTimeFormatData.localeOpt,
                          DateTimeFormat.relevantExtensionKeys,
                          localeData);
    internalProps.locale = r.locale;
    internalProps.calendar = r.ca;
    internalProps.numberingSystem = r.nu;
    var dataLocale = r.dataLocale;
    dataLocale = addUnicodeExtension(dataLocale, "-u-ca-" + r.ca);
    internalProps.timeZone = lazyDateTimeFormatData.timeZone;
    var formatOpt = lazyDateTimeFormatData.formatOpt;
    if (r.hc !== null && formatOpt.hour12 === undefined)
        formatOpt.hourCycle = r.hc;
    var skeleton;
    var pattern;
    if (lazyDateTimeFormatData.patternOption !== undefined) {
        pattern = lazyDateTimeFormatData.patternOption;
        skeleton = intl_skeletonForPattern(pattern);
        internalProps.patternOption = lazyDateTimeFormatData.patternOption;
    } else if (lazyDateTimeFormatData.dateStyle !== undefined ||
               lazyDateTimeFormatData.timeStyle !== undefined) {
        pattern = intl_patternForStyle(dataLocale,
                                       lazyDateTimeFormatData.dateStyle,
                                       lazyDateTimeFormatData.timeStyle,
                                       lazyDateTimeFormatData.timeZone,
                                       formatOpt.hour12,
                                       formatOpt.hourCycle);
        skeleton = intl_skeletonForPattern(pattern);
        internalProps.dateStyle = lazyDateTimeFormatData.dateStyle;
        internalProps.timeStyle = lazyDateTimeFormatData.timeStyle;
    } else {
        skeleton = toICUSkeleton(formatOpt);
        pattern = toBestICUPattern(dataLocale, skeleton, formatOpt);
    }
    internalProps.skeleton = skeleton;
    internalProps.pattern = pattern;
    return internalProps;
}
function getDateTimeFormatInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveDateTimeFormatInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function UnwrapDateTimeFormat(dtf) {
    if (IsObject(dtf) &&
        GuardToDateTimeFormat(dtf) === null &&
        !IsWrappedDateTimeFormat(dtf) &&
        callFunction(std_Object_isPrototypeOf, GetBuiltinPrototype("DateTimeFormat"), dtf))
    {
        dtf = dtf[intlFallbackSymbol()];
    }
    return dtf;
}
function CanonicalizeTimeZoneName(timeZone) {
    ;;
    ;;
    ;;
    var ianaTimeZone = intl_canonicalizeTimeZone(timeZone);
    ;;
    ;;
    if (ianaTimeZone === "Etc/UTC" || ianaTimeZone === "Etc/GMT") {
        ianaTimeZone = "UTC";
    }
    return ianaTimeZone;
}
var timeZoneCache = {
    icuDefaultTimeZone: undefined,
    defaultTimeZone: undefined,
};
function DefaultTimeZone() {
    if (intl_isDefaultTimeZone(timeZoneCache.icuDefaultTimeZone))
        return timeZoneCache.defaultTimeZone;
    var icuDefaultTimeZone = intl_defaultTimeZone();
    var timeZone = intl_IsValidTimeZoneName(icuDefaultTimeZone);
    if (timeZone === null) {
        const msPerHour = 60 * 60 * 1000;
        var offset = intl_defaultTimeZoneOffset();
        ;;
        var offsetHours = offset / msPerHour, offsetHoursFraction = offset % msPerHour;
        if (offsetHoursFraction === 0) {
            timeZone = "Etc/GMT" + (offsetHours < 0 ? "+" : "-") + std_Math_abs(offsetHours);
            timeZone = intl_IsValidTimeZoneName(timeZone);
        }
        if (timeZone === null)
            timeZone = "UTC";
    }
    var defaultTimeZone = CanonicalizeTimeZoneName(timeZone);
    timeZoneCache.defaultTimeZone = defaultTimeZone;
    timeZoneCache.icuDefaultTimeZone = icuDefaultTimeZone;
    return defaultTimeZone;
}
function InitializeDateTimeFormat(dateTimeFormat, thisValue, locales, options, mozExtensions) {
    ;;
    ;;
    var lazyDateTimeFormatData = std_Object_create(null);
    var requestedLocales = CanonicalizeLocaleList(locales);
    lazyDateTimeFormatData.requestedLocales = requestedLocales;
    options = ToDateTimeOptions(options, "any", "date");
    var localeOpt = new Record();
    lazyDateTimeFormatData.localeOpt = localeOpt;
    var localeMatcher =
        GetOption(options, "localeMatcher", "string", ["lookup", "best fit"],
                  "best fit");
    localeOpt.localeMatcher = localeMatcher;
    var calendar = GetOption(options, "calendar", "string", undefined, undefined);
    if (calendar !== undefined) {
        calendar = intl_ValidateAndCanonicalizeUnicodeExtensionType(calendar, "calendar", "ca");
    }
    localeOpt.ca = calendar;
    var numberingSystem = GetOption(options, "numberingSystem", "string", undefined, undefined);
    if (numberingSystem !== undefined) {
        numberingSystem = intl_ValidateAndCanonicalizeUnicodeExtensionType(numberingSystem,
                                                                           "numberingSystem",
                                                                           "nu");
    }
    localeOpt.nu = numberingSystem;
    var hr12 = GetOption(options, "hour12", "boolean", undefined, undefined);
    var hc = GetOption(options, "hourCycle", "string", ["h11", "h12", "h23", "h24"], undefined);
    if (hr12 !== undefined) {
        hc = null;
    }
    localeOpt.hc = hc;
    var tz = options.timeZone;
    if (tz !== undefined) {
        tz = ToString(tz);
        var timeZone = intl_IsValidTimeZoneName(tz);
        if (timeZone === null)
            ThrowRangeError(485, tz);
        tz = CanonicalizeTimeZoneName(timeZone);
    } else {
        tz = DefaultTimeZone();
    }
    lazyDateTimeFormatData.timeZone = tz;
    var formatOpt = new Record();
    lazyDateTimeFormatData.formatOpt = formatOpt;
    if (mozExtensions) {
        let pattern = GetOption(options, "pattern", "string", undefined, undefined);
        lazyDateTimeFormatData.patternOption = pattern;
    }
    formatOpt.weekday = GetOption(options, "weekday", "string", ["narrow", "short", "long"],
                                  undefined);
    formatOpt.era = GetOption(options, "era", "string", ["narrow", "short", "long"], undefined);
    formatOpt.year = GetOption(options, "year", "string", ["2-digit", "numeric"], undefined);
    formatOpt.month = GetOption(options, "month", "string",
                                ["2-digit", "numeric", "narrow", "short", "long"], undefined);
    formatOpt.day = GetOption(options, "day", "string", ["2-digit", "numeric"], undefined);
    formatOpt.hour = GetOption(options, "hour", "string", ["2-digit", "numeric"], undefined);
    formatOpt.minute = GetOption(options, "minute", "string", ["2-digit", "numeric"], undefined);
    formatOpt.second = GetOption(options, "second", "string", ["2-digit", "numeric"], undefined);
    formatOpt.fractionalSecondDigits = GetNumberOption(options, "fractionalSecondDigits", 1, 3,
                                                       undefined);
    formatOpt.timeZoneName = GetOption(options, "timeZoneName", "string", ["short", "long"],
                                       undefined);
    var formatMatcher =
        GetOption(options, "formatMatcher", "string", ["basic", "best fit"],
                  "best fit");
    void formatMatcher;
    var dateStyle = GetOption(options, "dateStyle", "string", ["full", "long", "medium", "short"],
                              undefined);
    lazyDateTimeFormatData.dateStyle = dateStyle;
    var timeStyle = GetOption(options, "timeStyle", "string", ["full", "long", "medium", "short"],
                              undefined);
    lazyDateTimeFormatData.timeStyle = timeStyle;
    if (dateStyle !== undefined || timeStyle !== undefined) {
      var optionsList = [
          "weekday", "era", "year", "month", "day", "hour", "minute", "second",
          "fractionalSecondDigits", "timeZoneName",
      ];
      for (var i = 0; i < optionsList.length; i++) {
          var option = optionsList[i];
          if (formatOpt[option] !== undefined) {
              ThrowTypeError(486, option,
                             dateStyle !== undefined ? "dateStyle" : "timeStyle");
          }
      }
    }
    if (hr12 !== undefined)
        formatOpt.hour12 = hr12;
    initializeIntlObject(dateTimeFormat, "DateTimeFormat", lazyDateTimeFormatData);
    if (dateTimeFormat !== thisValue &&
        callFunction(std_Object_isPrototypeOf, GetBuiltinPrototype("DateTimeFormat"), thisValue))
    {
        _DefineDataProperty(thisValue, intlFallbackSymbol(), dateTimeFormat,
                            0x08 | 0x10 | 0x20);
        return thisValue;
    }
    return dateTimeFormat;
}
function toICUSkeleton(options) {
    var skeleton = "";
    switch (options.weekday) {
    case "narrow":
        skeleton += "EEEEE";
        break;
    case "short":
        skeleton += "E";
        break;
    case "long":
        skeleton += "EEEE";
    }
    switch (options.era) {
    case "narrow":
        skeleton += "GGGGG";
        break;
    case "short":
        skeleton += "G";
        break;
    case "long":
        skeleton += "GGGG";
        break;
    }
    switch (options.year) {
    case "2-digit":
        skeleton += "yy";
        break;
    case "numeric":
        skeleton += "y";
        break;
    }
    switch (options.month) {
    case "2-digit":
        skeleton += "MM";
        break;
    case "numeric":
        skeleton += "M";
        break;
    case "narrow":
        skeleton += "MMMMM";
        break;
    case "short":
        skeleton += "MMM";
        break;
    case "long":
        skeleton += "MMMM";
        break;
    }
    switch (options.day) {
    case "2-digit":
        skeleton += "dd";
        break;
    case "numeric":
        skeleton += "d";
        break;
    }
    var hourSkeletonChar = "j";
    if (options.hour12 !== undefined) {
        if (options.hour12)
            hourSkeletonChar = "h";
        else
            hourSkeletonChar = "H";
    } else {
        switch (options.hourCycle) {
        case "h11":
        case "h12":
            hourSkeletonChar = "h";
            break;
        case "h23":
        case "h24":
            hourSkeletonChar = "H";
            break;
        }
    }
    switch (options.hour) {
    case "2-digit":
        skeleton += hourSkeletonChar + hourSkeletonChar;
        break;
    case "numeric":
        skeleton += hourSkeletonChar;
        break;
    }
    switch (options.minute) {
    case "2-digit":
        skeleton += "mm";
        break;
    case "numeric":
        skeleton += "m";
        break;
    }
    switch (options.second) {
    case "2-digit":
        skeleton += "ss";
        break;
    case "numeric":
        skeleton += "s";
        break;
    }
    switch (options.fractionalSecondDigits) {
    case 1:
        skeleton += "S";
        break;
    case 2:
        skeleton += "SS";
        break;
    case 3:
        skeleton += "SSS";
        break;
    }
    switch (options.timeZoneName) {
    case "short":
        skeleton += "z";
        break;
    case "long":
        skeleton += "zzzz";
        break;
    }
    return skeleton;
}
function toBestICUPattern(locale, skeleton, options) {
    return intl_patternForSkeleton(locale, skeleton, options.hourCycle);
}
function ToDateTimeOptions(options, required, defaults) {
    ;;
    ;;
    if (options === undefined)
        options = null;
    else
        options = ToObject(options);
    options = std_Object_create(options);
    var needDefaults = true;
    if (required === "date" || required === "any") {
        if (options.weekday !== undefined)
            needDefaults = false;
        if (options.year !== undefined)
            needDefaults = false;
        if (options.month !== undefined)
            needDefaults = false;
        if (options.day !== undefined)
            needDefaults = false;
    }
    if (required === "time" || required === "any") {
        if (options.hour !== undefined)
            needDefaults = false;
        if (options.minute !== undefined)
            needDefaults = false;
        if (options.second !== undefined)
            needDefaults = false;
        if (options.fractionalSecondDigits !== undefined)
            needDefaults = false;
    }
    var dateStyle = options.dateStyle;
    var timeStyle = options.timeStyle;
    if (dateStyle !== undefined || timeStyle !== undefined)
        needDefaults = false;
    if (required === "date" && timeStyle !== undefined)
        ThrowTypeError(487, "timeStyle", "toLocaleDateString");
    if (required === "time" && dateStyle !== undefined)
        ThrowTypeError(487, "dateStyle", "toLocaleTimeString");
    if (needDefaults && (defaults === "date" || defaults === "all")) {
        _DefineDataProperty(options, "year", "numeric");
        _DefineDataProperty(options, "month", "numeric");
        _DefineDataProperty(options, "day", "numeric");
    }
    if (needDefaults && (defaults === "time" || defaults === "all")) {
        _DefineDataProperty(options, "hour", "numeric");
        _DefineDataProperty(options, "minute", "numeric");
        _DefineDataProperty(options, "second", "numeric");
    }
    return options;
}
function Intl_DateTimeFormat_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "DateTimeFormat";
    var requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
var dateTimeFormatInternalProperties = {
    localeData: dateTimeFormatLocaleData,
    relevantExtensionKeys: ["ca", "hc", "nu"],
};
function dateTimeFormatLocaleData() {
    return {
        ca: intl_availableCalendars,
        nu: getNumberingSystems,
        hc: () => {
            return [null, "h11", "h12", "h23", "h24"];
        },
        default: {
            ca: intl_defaultCalendar,
            nu: intl_numberingSystem,
            hc: () => {
                return null;
            },
        },
    };
}
function createDateTimeFormatFormat(dtf) {
    return function(date) {
        ;;
        ;;
        var x = (date === undefined) ? std_Date_now() : ToNumber(date);
        return intl_FormatDateTime(dtf, x, false);
    };
}
function $Intl_DateTimeFormat_format_get() {
    var thisArg = UnwrapDateTimeFormat(this);
    var dtf = thisArg;
    if (!IsObject(dtf) || (dtf = GuardToDateTimeFormat(dtf)) === null) {
        return callFunction(CallDateTimeFormatMethodIfWrapped, thisArg,
                            "$Intl_DateTimeFormat_format_get");
    }
    var internals = getDateTimeFormatInternals(dtf);
    if (internals.boundFormat === undefined) {
        internals.boundFormat = createDateTimeFormatFormat(dtf);
    }
    return internals.boundFormat;
}
_SetCanonicalName($Intl_DateTimeFormat_format_get, "get format");
function Intl_DateTimeFormat_formatToParts(date) {
    var dtf = this;
    if (!IsObject(dtf) || (dtf = GuardToDateTimeFormat(dtf)) === null) {
        return callFunction(CallDateTimeFormatMethodIfWrapped, this, date,
                            "Intl_DateTimeFormat_formatToParts");
    }
    var x = (date === undefined) ? std_Date_now() : ToNumber(date);
    getDateTimeFormatInternals(dtf);
    return intl_FormatDateTime(dtf, x, true);
}
function Intl_DateTimeFormat_formatRange(startDate, endDate) {
    var dtf = this;
    if (!IsObject(dtf) || (dtf = GuardToDateTimeFormat(dtf)) === null) {
        return callFunction(CallDateTimeFormatMethodIfWrapped, this, startDate, endDate,
                            "Intl_DateTimeFormat_formatRange");
    }
    if (startDate === undefined || endDate === undefined) {
        ThrowTypeError(490, startDate === undefined ? "start" : "end",
                       "formatRange");
    }
    var x = ToNumber(startDate);
    var y = ToNumber(endDate);
    if (x > y) {
        ThrowRangeError(492, "formatRange");
    }
    getDateTimeFormatInternals(dtf);
    return intl_FormatDateTimeRange(dtf, x, y, false);
}
function Intl_DateTimeFormat_formatRangeToParts(startDate, endDate) {
    var dtf = this;
    if (!IsObject(dtf) || (dtf = GuardToDateTimeFormat(dtf)) === null) {
        return callFunction(CallDateTimeFormatMethodIfWrapped, this, startDate, endDate,
                            "Intl_DateTimeFormat_formatRangeToParts");
    }
    if (startDate === undefined || endDate === undefined) {
        ThrowTypeError(490, startDate === undefined ? "start" : "end",
                       "formatRangeToParts");
    }
    var x = ToNumber(startDate);
    var y = ToNumber(endDate);
    if (x > y) {
        ThrowRangeError(492, "formatRangeToParts");
    }
    getDateTimeFormatInternals(dtf);
    return intl_FormatDateTimeRange(dtf, x, y, true);
}
function Intl_DateTimeFormat_resolvedOptions() {
    var thisArg = UnwrapDateTimeFormat(this);
    var dtf = thisArg;
    if (!IsObject(dtf) || (dtf = GuardToDateTimeFormat(dtf)) === null) {
        return callFunction(CallDateTimeFormatMethodIfWrapped, thisArg,
                            "Intl_DateTimeFormat_resolvedOptions");
    }
    var internals = getDateTimeFormatInternals(dtf);
    var result = {
        locale: internals.locale,
        calendar: internals.calendar,
        numberingSystem: internals.numberingSystem,
        timeZone: internals.timeZone,
    };
    if (internals.patternOption !== undefined) {
        _DefineDataProperty(result, "pattern", internals.pattern);
    }
    var hasDateStyle = internals.dateStyle !== undefined;
    var hasTimeStyle = internals.timeStyle !== undefined;
    if (hasDateStyle || hasTimeStyle) {
        if (hasTimeStyle) {
            resolveICUPattern(internals.pattern, result, false);
        }
        if (hasDateStyle) {
            _DefineDataProperty(result, "dateStyle", internals.dateStyle);
        }
        if (hasTimeStyle) {
            _DefineDataProperty(result, "timeStyle", internals.timeStyle);
        }
    } else {
        resolveICUPattern(internals.pattern, result, true);
    }
    return result;
}
function resolveICUPattern(pattern, result, includeDateTimeFields) {
    ;;
    var hourCycle, weekday, era, year, month, day, dayPeriod, hour, minute, second,
        fractionalSecondDigits, timeZoneName;
    var i = 0;
    while (i < pattern.length) {
        var c = pattern[i++];
        if (c === "'") {
            while (i < pattern.length && pattern[i] !== "'")
                i++;
            i++;
        } else {
            var count = 1;
            while (i < pattern.length && pattern[i] === c) {
                i++;
                count++;
            }
            var value;
            switch (c) {
            case "G":
            case "E":
            case "c":
            case "B":
            case "z":
            case "v":
            case "V":
                if (count <= 3)
                    value = "short";
                else if (count === 4)
                    value = "long";
                else
                    value = "narrow";
                break;
            case "y":
            case "d":
            case "h":
            case "H":
            case "m":
            case "s":
            case "k":
            case "K":
                if (count === 2)
                    value = "2-digit";
                else
                    value = "numeric";
                break;
            case "M":
            case "L":
                if (count === 1)
                    value = "numeric";
                else if (count === 2)
                    value = "2-digit";
                else if (count === 3)
                    value = "short";
                else if (count === 4)
                    value = "long";
                else
                    value = "narrow";
                break;
            case "S":
                value = count;
                break;
            default:
            }
            switch (c) {
            case "E":
            case "c":
                weekday = value;
                break;
            case "G":
                era = value;
                break;
            case "y":
                year = value;
                break;
            case "M":
            case "L":
                month = value;
                break;
            case "d":
                day = value;
                break;
            case "B":
                dayPeriod = value;
                break;
            case "h":
                hourCycle = "h12";
                hour = value;
                break;
            case "H":
                hourCycle = "h23";
                hour = value;
                break;
            case "k":
                hourCycle = "h24";
                hour = value;
                break;
            case "K":
                hourCycle = "h11";
                hour = value;
                break;
            case "m":
                minute = value;
                break;
            case "s":
                second = value;
                break;
            case "S":
                fractionalSecondDigits = value;
                break;
            case "z":
            case "v":
            case "V":
                timeZoneName = value;
                break;
            }
        }
    }
    if (hourCycle) {
        _DefineDataProperty(result, "hourCycle", hourCycle);
        _DefineDataProperty(result, "hour12", hourCycle === "h11" || hourCycle === "h12");
    }
    if (!includeDateTimeFields) {
        return;
    }
    if (weekday) {
        _DefineDataProperty(result, "weekday", weekday);
    }
    if (era) {
        _DefineDataProperty(result, "era", era);
    }
    if (year) {
        _DefineDataProperty(result, "year", year);
    }
    if (month) {
        _DefineDataProperty(result, "month", month);
    }
    if (day) {
        _DefineDataProperty(result, "day", day);
    }
    if (hour) {
        _DefineDataProperty(result, "hour", hour);
    }
    if (minute) {
        _DefineDataProperty(result, "minute", minute);
    }
    if (second) {
        _DefineDataProperty(result, "second", second);
    }
    if (fractionalSecondDigits) {
        _DefineDataProperty(result, "fractionalSecondDigits", fractionalSecondDigits);
    }
    if (timeZoneName) {
        _DefineDataProperty(result, "timeZoneName", timeZoneName);
    }
}
var displayNamesInternalProperties = {
    localeData: function()
    {
        return {};
    },
    relevantExtensionKeys: []
};
var mozDisplayNamesInternalProperties = {
    localeData: function()
    {
        return {
            ca: intl_availableCalendars,
            default: {
                ca: intl_defaultCalendar,
            },
        };
    },
    relevantExtensionKeys: ["ca"]
};
function resolveDisplayNamesInternals(lazyDisplayNamesData) {
    ;;
    var internalProps = std_Object_create(null);
    var mozExtensions = lazyDisplayNamesData.mozExtensions;
    var DisplayNames = mozExtensions ?
                       mozDisplayNamesInternalProperties :
                       displayNamesInternalProperties;
    var localeData = DisplayNames.localeData;
    var r = ResolveLocale("DisplayNames",
                          lazyDisplayNamesData.requestedLocales,
                          lazyDisplayNamesData.opt,
                          DisplayNames.relevantExtensionKeys,
                          localeData);
    internalProps.style = lazyDisplayNamesData.style;
    internalProps.type = lazyDisplayNamesData.type;
    internalProps.fallback = lazyDisplayNamesData.fallback;
    internalProps.locale = r.locale;
    if (mozExtensions) {
        internalProps.calendar = r.ca;
    }
    return internalProps;
}
function getDisplayNamesInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveDisplayNamesInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function InitializeDisplayNames(displayNames, locales, options, mozExtensions) {
    ;;
    ;;
    var lazyDisplayNamesData = std_Object_create(null);
    var requestedLocales = CanonicalizeLocaleList(locales);
    lazyDisplayNamesData.requestedLocales = requestedLocales;
    options = ToObject(options);
    var opt = new Record();
    lazyDisplayNamesData.opt = opt;
    lazyDisplayNamesData.mozExtensions = mozExtensions;
    var matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    if (mozExtensions) {
        var calendar = GetOption(options, "calendar", "string", undefined, undefined);
        if (calendar !== undefined) {
            calendar = intl_ValidateAndCanonicalizeUnicodeExtensionType(calendar, "calendar", "ca");
        }
        opt.ca = calendar;
    }
    var style = GetOption(options, "style", "string", ["narrow", "short", "long"], "long");
    lazyDisplayNamesData.style = style;
    var type;
    if (mozExtensions) {
        type = GetOption(options, "type", "string",
                         ["language", "region", "script", "currency", "weekday", "month",
                          "quarter", "dayPeriod", "dateTimeField"], undefined);
    } else {
        type = GetOption(options, "type", "string",
                         ["language", "region", "script", "currency"], undefined);
    }
    if (type === undefined) {
        ThrowTypeError(491);
    }
    lazyDisplayNamesData.type = type;
    var fallback = GetOption(options, "fallback", "string", ["code", "none"], "code");
    lazyDisplayNamesData.fallback = fallback;
    initializeIntlObject(displayNames, "DisplayNames", lazyDisplayNamesData);
}
function Intl_DisplayNames_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "DisplayNames";
    var requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
function Intl_DisplayNames_of(code) {
  var displayNames = this;
  if (!IsObject(displayNames) || (displayNames = GuardToDisplayNames(displayNames)) === null) {
      return callFunction(CallDisplayNamesMethodIfWrapped, this, "Intl_DisplayNames_of");
  }
  code = ToString(code);
  var internals = getDisplayNamesInternals(displayNames);
  var {locale, calendar = "", style, type, fallback} = internals;
  return intl_ComputeDisplayName(displayNames, locale, calendar, style, fallback, type, code);
}
function Intl_DisplayNames_resolvedOptions() {
    var displayNames = this;
    if (!IsObject(displayNames) || (displayNames = GuardToDisplayNames(displayNames)) === null) {
        return callFunction(CallDisplayNamesMethodIfWrapped, this,
                            "Intl_DisplayNames_resolvedOptions");
    }
    var internals = getDisplayNamesInternals(displayNames);
    var options = {
        locale: internals.locale,
        style: internals.style,
        type: internals.type,
        fallback: internals.fallback,
    };
    if (hasOwn("calendar", internals)) {
        options.calendar = internals.calendar;
    }
    return options;
}
function Intl_getCanonicalLocales(locales) {
    return CanonicalizeLocaleList(locales);
}
function Intl_getCalendarInfo(locales) {
    const requestedLocales = CanonicalizeLocaleList(locales);
    const DateTimeFormat = dateTimeFormatInternalProperties;
    const localeData = DateTimeFormat.localeData;
    const localeOpt = new Record();
    localeOpt.localeMatcher = "best fit";
    const r = ResolveLocale("DateTimeFormat",
                            requestedLocales,
                            localeOpt,
                            DateTimeFormat.relevantExtensionKeys,
                            localeData);
    const result = intl_GetCalendarInfo(r.locale);
    _DefineDataProperty(result, "calendar", r.ca);
    _DefineDataProperty(result, "locale", r.locale);
    return result;
}
function Intl_getDisplayNames(locales, options) {
    const requestedLocales = CanonicalizeLocaleList(locales);
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    const DateTimeFormat = dateTimeFormatInternalProperties;
    const localeData = DateTimeFormat.localeData;
    const localeOpt = new Record();
    localeOpt.localeMatcher = "best fit";
    const r = ResolveLocale("DateTimeFormat",
                            requestedLocales,
                            localeOpt,
                            DateTimeFormat.relevantExtensionKeys,
                            localeData);
    const style = GetOption(options, "style", "string", ["long", "short", "narrow"], "long");
    let keys = options.keys;
    if (keys === undefined) {
        keys = [];
    } else if (!IsObject(keys)) {
        ThrowTypeError(479);
    }
    let processedKeys = [];
    let len = ToLength(keys.length);
    for (let i = 0; i < len; i++) {
        _DefineDataProperty(processedKeys, i, ToString(keys[i]));
    }
    const names = intl_ComputeDisplayNames(r.locale, style, processedKeys);
    const values = {};
    for (let i = 0; i < len; i++) {
        const key = processedKeys[i];
        const name = names[i];
        ;;
        ;;
        _DefineDataProperty(values, key, name);
    }
    const result = { locale: r.locale, style, values };
    return result;
}
function Intl_getLocaleInfo(locales) {
  const requestedLocales = CanonicalizeLocaleList(locales);
  const DateTimeFormat = dateTimeFormatInternalProperties;
  const localeData = DateTimeFormat.localeData;
  const localeOpt = new Record();
  localeOpt.localeMatcher = "best fit";
  const r = ResolveLocale("DateTimeFormat",
                          requestedLocales,
                          localeOpt,
                          DateTimeFormat.relevantExtensionKeys,
                          localeData);
  return intl_GetLocaleInfo(r.locale);
}
var listFormatInternalProperties = {
    localeData: function()
    {
        return {};
    },
    relevantExtensionKeys: []
};
function resolveListFormatInternals(lazyListFormatData) {
    ;;
    var internalProps = std_Object_create(null);
    var ListFormat = listFormatInternalProperties;
    var localeData = ListFormat.localeData;
    var r = ResolveLocale("ListFormat",
                          lazyListFormatData.requestedLocales,
                          lazyListFormatData.opt,
                          ListFormat.relevantExtensionKeys,
                          localeData);
    internalProps.locale = r.locale;
    internalProps.type = lazyListFormatData.type;
    internalProps.style = lazyListFormatData.style;
    return internalProps;
}
function getListFormatInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveListFormatInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function InitializeListFormat(listFormat, locales, options, supportsTypeAndStyle) {
    ;;
    ;;
    var lazyListFormatData = std_Object_create(null);
    var requestedLocales = CanonicalizeLocaleList(locales);
    lazyListFormatData.requestedLocales = requestedLocales;
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    var opt = new Record();
    lazyListFormatData.opt = opt;
    let matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    var type = GetOption(options, "type", "string",
                         supportsTypeAndStyle ? ["conjunction", "disjunction", "unit"] : ["conjunction"],
                         "conjunction");
    lazyListFormatData.type = type;
    var style = GetOption(options, "style", "string",
                          supportsTypeAndStyle ? ["long", "short", "narrow"] : ["long"],
                          "long");
    lazyListFormatData.style = style;
    initializeIntlObject(listFormat, "ListFormat", lazyListFormatData);
}
function Intl_ListFormat_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "ListFormat";
    var requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
function StringListFromIterable(iterable, methodName) {
    if (iterable === undefined) {
        return [];
    }
    var list = [];
    for (var element of allowContentIter(iterable)) {
        if (typeof element !== "string") {
            ThrowTypeError(64, methodName, "string", typeof element);
        }
        _DefineDataProperty(list, list.length, element);
    }
    return list;
}
function Intl_ListFormat_format(list) {
    var listFormat = this;
    if (!IsObject(listFormat) || (listFormat = GuardToListFormat(listFormat)) === null) {
        return callFunction(CallListFormatMethodIfWrapped, this, list,
                            "Intl_ListFormat_format");
    }
    var stringList = StringListFromIterable(list, "format");
    if (stringList.length < 2) {
        return stringList.length === 0 ? "" : stringList[0];
    }
    getListFormatInternals(listFormat);
    return intl_FormatList(listFormat, stringList, false);
}
function Intl_ListFormat_formatToParts(list) {
    var listFormat = this;
    if (!IsObject(listFormat) || (listFormat = GuardToListFormat(listFormat)) === null) {
        return callFunction(CallListFormatMethodIfWrapped, this, list,
                            "Intl_ListFormat_formatToParts");
    }
    var stringList = StringListFromIterable(list, "formatToParts");
    if (stringList.length < 2) {
        return stringList.length === 0 ? [] : [{type: "element", value: stringList[0]}];
    }
    getListFormatInternals(listFormat);
    return intl_FormatList(listFormat, stringList, true);
}
function Intl_ListFormat_resolvedOptions() {
    var listFormat = this;
    if (!IsObject(listFormat) || (listFormat = GuardToListFormat(listFormat)) === null) {
        return callFunction(CallListFormatMethodIfWrapped, this,
                            "Intl_ListFormat_resolvedOptions");
    }
    var internals = getListFormatInternals(listFormat);
    var result = {
        locale: internals.locale,
        type: internals.type,
        style: internals.style,
    };
    return result;
}
var numberFormatInternalProperties = {
    localeData: numberFormatLocaleData,
    relevantExtensionKeys: ["nu"],
};
function resolveNumberFormatInternals(lazyNumberFormatData) {
    ;;
    var internalProps = std_Object_create(null);
    var NumberFormat = numberFormatInternalProperties;
    var localeData = NumberFormat.localeData;
    var r = ResolveLocale("NumberFormat",
                          lazyNumberFormatData.requestedLocales,
                          lazyNumberFormatData.opt,
                          NumberFormat.relevantExtensionKeys,
                          localeData);
    internalProps.locale = r.locale;
    internalProps.numberingSystem = r.nu;
    var style = lazyNumberFormatData.style;
    internalProps.style = style;
    if (style === "currency") {
        internalProps.currency = lazyNumberFormatData.currency;
        internalProps.currencyDisplay = lazyNumberFormatData.currencyDisplay;
        internalProps.currencySign = lazyNumberFormatData.currencySign;
    }
    if (style === "unit") {
        internalProps.unit = lazyNumberFormatData.unit;
        internalProps.unitDisplay = lazyNumberFormatData.unitDisplay;
    }
    var notation = lazyNumberFormatData.notation;
    internalProps.notation = notation;
    internalProps.minimumIntegerDigits = lazyNumberFormatData.minimumIntegerDigits;
    if ("minimumFractionDigits" in lazyNumberFormatData) {
        ;;
        internalProps.minimumFractionDigits = lazyNumberFormatData.minimumFractionDigits;
        internalProps.maximumFractionDigits = lazyNumberFormatData.maximumFractionDigits;
    }
    if ("minimumSignificantDigits" in lazyNumberFormatData) {
        ;;
        internalProps.minimumSignificantDigits = lazyNumberFormatData.minimumSignificantDigits;
        internalProps.maximumSignificantDigits = lazyNumberFormatData.maximumSignificantDigits;
    }
    if (notation === "compact")
        internalProps.compactDisplay = lazyNumberFormatData.compactDisplay;
    internalProps.useGrouping = lazyNumberFormatData.useGrouping;
    internalProps.signDisplay = lazyNumberFormatData.signDisplay;
    return internalProps;
}
function getNumberFormatInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveNumberFormatInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function UnwrapNumberFormat(nf) {
    if (IsObject(nf) &&
        GuardToNumberFormat(nf) === null &&
        !IsWrappedNumberFormat(nf) &&
        callFunction(std_Object_isPrototypeOf, GetBuiltinPrototype("NumberFormat"), nf))
    {
        nf = nf[intlFallbackSymbol()];
    }
    return nf;
}
function SetNumberFormatDigitOptions(lazyData, options, mnfdDefault, mxfdDefault, notation) {
    ;;
    ;;
    ;;
    ;;
    ;;
    const mnid = GetNumberOption(options, "minimumIntegerDigits", 1, 21, 1);
    let mnfd = options.minimumFractionDigits;
    let mxfd = options.maximumFractionDigits;
    let mnsd = options.minimumSignificantDigits;
    let mxsd = options.maximumSignificantDigits;
    lazyData.minimumIntegerDigits = mnid;
    if (mnsd !== undefined || mxsd !== undefined) {
        mnsd = DefaultNumberOption(mnsd, 1, 21, 1);
        mxsd = DefaultNumberOption(mxsd, mnsd, 21, 21);
        lazyData.minimumSignificantDigits = mnsd;
        lazyData.maximumSignificantDigits = mxsd;
    }
    else if (mnfd !== undefined || mxfd !== undefined) {
        mnfd = DefaultNumberOption(mnfd, 0, 20, undefined);
        mxfd = DefaultNumberOption(mxfd, 0, 20, undefined);
        if (mnfd === undefined) {
            ;;
            mnfd = std_Math_min(mnfdDefault, mxfd);
        }
        else if (mxfd === undefined) {
            mxfd = std_Math_max(mxfdDefault, mnfd);
        }
        else if (mnfd > mxfd) {
            ThrowRangeError(478, mxfd);
        }
        lazyData.minimumFractionDigits = mnfd;
        lazyData.maximumFractionDigits = mxfd;
    }
    else if (notation === "compact") {
    }
    else {
        lazyData.minimumFractionDigits = mnfdDefault;
        lazyData.maximumFractionDigits = mxfdDefault;
    }
}
function toASCIIUpperCase(s) {
    ;;
    var result = "";
    for (var i = 0; i < s.length; i++) {
        var c = callFunction(std_String_charCodeAt, s, i);
        result += (0x61 <= c && c <= 0x7A)
                  ? callFunction(std_String_fromCharCode, null, c & ~0x20)
                  : s[i];
    }
    return result;
}
function IsWellFormedCurrencyCode(currency) {
    ;;
    return currency.length === 3 && IsASCIIAlphaString(currency);
}
function IsWellFormedUnitIdentifier(unitIdentifier) {
    ;;
    if (IsSanctionedSimpleUnitIdentifier(unitIdentifier))
        return true;
    var pos = callFunction(std_String_indexOf, unitIdentifier, "-per-");
    if (pos < 0)
        return false;
    var next = pos + "-per-".length;
    var numerator = Substring(unitIdentifier, 0, pos);
    var denominator = Substring(unitIdentifier, next, unitIdentifier.length - next);
    return IsSanctionedSimpleUnitIdentifier(numerator) &&
           IsSanctionedSimpleUnitIdentifier(denominator);
}
function IsSanctionedSimpleUnitIdentifier(unitIdentifier) {
    ;;
    var isSanctioned = hasOwn(unitIdentifier, sanctionedSimpleUnitIdentifiers);
    return isSanctioned;
}
function InitializeNumberFormat(numberFormat, thisValue, locales, options) {
    ;;
    ;;
    var lazyNumberFormatData = std_Object_create(null);
    var requestedLocales = CanonicalizeLocaleList(locales);
    lazyNumberFormatData.requestedLocales = requestedLocales;
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    var opt = new Record();
    lazyNumberFormatData.opt = opt;
    var matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    var numberingSystem = GetOption(options, "numberingSystem", "string", undefined, undefined);
    if (numberingSystem !== undefined) {
        numberingSystem = intl_ValidateAndCanonicalizeUnicodeExtensionType(numberingSystem,
                                                                           "numberingSystem",
                                                                           "nu");
    }
    opt.nu = numberingSystem;
    var style = GetOption(options, "style", "string", ["decimal", "percent", "currency", "unit"],
                          "decimal");
    lazyNumberFormatData.style = style;
    var currency = GetOption(options, "currency", "string", undefined, undefined);
    if (currency !== undefined && !IsWellFormedCurrencyCode(currency))
        ThrowRangeError(476, currency);
    var cDigits;
    if (style === "currency") {
        if (currency === undefined)
            ThrowTypeError(488);
        currency = toASCIIUpperCase(currency);
        lazyNumberFormatData.currency = currency;
        cDigits = CurrencyDigits(currency);
    }
    var currencyDisplay = GetOption(options, "currencyDisplay", "string",
                                    ["code", "symbol", "narrowSymbol", "name"], "symbol");
    if (style === "currency")
        lazyNumberFormatData.currencyDisplay = currencyDisplay;
    var currencySign = GetOption(options, "currencySign", "string", ["standard", "accounting"],
                                 "standard");
    if (style === "currency")
        lazyNumberFormatData.currencySign = currencySign;
    var unit = GetOption(options, "unit", "string", undefined, undefined);
    if (unit !== undefined && !IsWellFormedUnitIdentifier(unit))
        ThrowRangeError(477, unit);
    var unitDisplay = GetOption(options, "unitDisplay", "string",
                                ["short", "narrow", "long"], "short");
    if (style === "unit") {
        if (unit === undefined)
            ThrowTypeError(489);
        lazyNumberFormatData.unit = unit;
        lazyNumberFormatData.unitDisplay = unitDisplay;
    }
    var mnfdDefault, mxfdDefault;
    if (style === "currency") {
        mnfdDefault = cDigits;
        mxfdDefault = cDigits;
    } else {
        mnfdDefault = 0;
        mxfdDefault = style === "percent" ? 0 : 3;
    }
    var notation = GetOption(options, "notation", "string",
                             ["standard", "scientific", "engineering", "compact"], "standard");
    lazyNumberFormatData.notation = notation;
    SetNumberFormatDigitOptions(lazyNumberFormatData, options, mnfdDefault, mxfdDefault, notation);
    var compactDisplay = GetOption(options, "compactDisplay", "string",
                                   ["short", "long"], "short");
    if (notation === "compact")
        lazyNumberFormatData.compactDisplay = compactDisplay;
    var useGrouping = GetOption(options, "useGrouping", "boolean", undefined, true);
    lazyNumberFormatData.useGrouping = useGrouping;
    var signDisplay = GetOption(options, "signDisplay", "string",
                                ["auto", "never", "always", "exceptZero"], "auto");
    lazyNumberFormatData.signDisplay = signDisplay;
    initializeIntlObject(numberFormat, "NumberFormat", lazyNumberFormatData);
    if (numberFormat !== thisValue &&
        callFunction(std_Object_isPrototypeOf, GetBuiltinPrototype("NumberFormat"), thisValue))
    {
        _DefineDataProperty(thisValue, intlFallbackSymbol(), numberFormat,
                            0x08 | 0x10 | 0x20);
        return thisValue;
    }
    return numberFormat;
}
function CurrencyDigits(currency) {
    ;;
    ;;
    ;;
    if (hasOwn(currency, currencyDigits))
        return currencyDigits[currency];
    return 2;
}
function Intl_NumberFormat_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "NumberFormat";
    var requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
function getNumberingSystems(locale) {
    var defaultNumberingSystem = intl_numberingSystem(locale);
    return [
        defaultNumberingSystem,
        "adlm", "ahom", "arab", "arabext", "bali", "beng", "bhks", "brah", "cakm", "cham", "deva", "diak", "fullwide", "gong", "gonm", "gujr", "guru", "hanidec", "hmng", "hmnp", "java", "kali", "khmr", "knda", "lana", "lanatham", "laoo", "latn", "lepc", "limb", "mathbold", "mathdbl", "mathmono", "mathsanb", "mathsans", "mlym", "modi", "mong", "mroo", "mtei", "mymr", "mymrshan", "mymrtlng", "newa", "nkoo", "olck", "orya", "osma", "rohg", "saur", "segment", "shrd", "sind", "sinh", "sora", "sund", "takr", "talu", "tamldec", "telu", "thai", "tibt", "tirh", "vaii", "wara", "wcho"
    ];
}
function numberFormatLocaleData() {
    return {
        nu: getNumberingSystems,
        default: {
            nu: intl_numberingSystem,
        },
    };
}
function createNumberFormatFormat(nf) {
    return function(value) {
        ;;
        ;;
        var x = ToNumeric(value);
        return intl_FormatNumber(nf, x, false, false);
    };
}
function $Intl_NumberFormat_format_get() {
    var thisArg = UnwrapNumberFormat(this);
    var nf = thisArg;
    if (!IsObject(nf) || (nf = GuardToNumberFormat(nf)) === null) {
        return callFunction(CallNumberFormatMethodIfWrapped, thisArg,
                            "$Intl_NumberFormat_format_get");
    }
    var internals = getNumberFormatInternals(nf);
    if (internals.boundFormat === undefined) {
        internals.boundFormat = createNumberFormatFormat(nf);
    }
    return internals.boundFormat;
}
_SetCanonicalName($Intl_NumberFormat_format_get, "get format");
function Intl_NumberFormat_formatToParts(value) {
    var nf = this;
    if (!IsObject(nf) || (nf = GuardToNumberFormat(nf)) === null) {
        return callFunction(CallNumberFormatMethodIfWrapped, this, value,
                            "Intl_NumberFormat_formatToParts");
    }
    var x = ToNumeric(value);
    var internals = getNumberFormatInternals(nf);
    var unitStyle = internals.style === "unit";
    return intl_FormatNumber(nf, x, true, unitStyle);
}
function Intl_NumberFormat_resolvedOptions() {
    var thisArg = UnwrapNumberFormat(this);
    var nf = thisArg;
    if (!IsObject(nf) || (nf = GuardToNumberFormat(nf)) === null) {
        return callFunction(CallNumberFormatMethodIfWrapped, thisArg,
                            "Intl_NumberFormat_resolvedOptions");
    }
    var internals = getNumberFormatInternals(nf);
    var result = {
        locale: internals.locale,
        numberingSystem: internals.numberingSystem,
        style: internals.style,
    };
    ;;
    ;;
    ;;
    if (hasOwn("currency", internals)) {
        _DefineDataProperty(result, "currency", internals.currency);
        _DefineDataProperty(result, "currencyDisplay", internals.currencyDisplay);
        _DefineDataProperty(result, "currencySign", internals.currencySign);
    }
    ;;
    ;;
    if (hasOwn("unit", internals)) {
        _DefineDataProperty(result, "unit", internals.unit);
        _DefineDataProperty(result, "unitDisplay", internals.unitDisplay);
    }
    _DefineDataProperty(result, "minimumIntegerDigits", internals.minimumIntegerDigits);
    ;;
    if (hasOwn("minimumFractionDigits", internals)) {
        _DefineDataProperty(result, "minimumFractionDigits", internals.minimumFractionDigits);
        _DefineDataProperty(result, "maximumFractionDigits", internals.maximumFractionDigits);
    }
    ;;
    if (hasOwn("minimumSignificantDigits", internals)) {
        _DefineDataProperty(result, "minimumSignificantDigits",
                            internals.minimumSignificantDigits);
        _DefineDataProperty(result, "maximumSignificantDigits",
                            internals.maximumSignificantDigits);
    }
    _DefineDataProperty(result, "useGrouping", internals.useGrouping);
    var notation = internals.notation;
    _DefineDataProperty(result, "notation", notation);
    if (notation === "compact")
        _DefineDataProperty(result, "compactDisplay", internals.compactDisplay);
    _DefineDataProperty(result, "signDisplay", internals.signDisplay);
    return result;
}
var pluralRulesInternalProperties = {
    localeData: pluralRulesLocaleData,
    relevantExtensionKeys: [],
};
function pluralRulesLocaleData() {
    return {};
}
function resolvePluralRulesInternals(lazyPluralRulesData) {
    ;;
    var internalProps = std_Object_create(null);
    var PluralRules = pluralRulesInternalProperties;
    var localeData = PluralRules.localeData;
    const r = ResolveLocale("PluralRules",
                            lazyPluralRulesData.requestedLocales,
                            lazyPluralRulesData.opt,
                            PluralRules.relevantExtensionKeys,
                            localeData);
    internalProps.locale = r.locale;
    internalProps.type = lazyPluralRulesData.type;
    internalProps.minimumIntegerDigits = lazyPluralRulesData.minimumIntegerDigits;
    if ("minimumFractionDigits" in lazyPluralRulesData) {
        ;;
        internalProps.minimumFractionDigits = lazyPluralRulesData.minimumFractionDigits;
        internalProps.maximumFractionDigits = lazyPluralRulesData.maximumFractionDigits;
    }
    if ("minimumSignificantDigits" in lazyPluralRulesData) {
        ;;
        internalProps.minimumSignificantDigits = lazyPluralRulesData.minimumSignificantDigits;
        internalProps.maximumSignificantDigits = lazyPluralRulesData.maximumSignificantDigits;
    }
    internalProps.pluralCategories = null;
    return internalProps;
}
function getPluralRulesInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolvePluralRulesInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function InitializePluralRules(pluralRules, locales, options) {
    ;;
    ;;
    const lazyPluralRulesData = std_Object_create(null);
    let requestedLocales = CanonicalizeLocaleList(locales);
    lazyPluralRulesData.requestedLocales = requestedLocales;
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    let opt = new Record();
    lazyPluralRulesData.opt = opt;
    let matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    const type = GetOption(options, "type", "string", ["cardinal", "ordinal"], "cardinal");
    lazyPluralRulesData.type = type;
    SetNumberFormatDigitOptions(lazyPluralRulesData, options, 0, 3, "standard");
    initializeIntlObject(pluralRules, "PluralRules", lazyPluralRulesData);
}
function Intl_PluralRules_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "PluralRules";
    let requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
function Intl_PluralRules_select(value) {
    let pluralRules = this;
    if (!IsObject(pluralRules) || (pluralRules = GuardToPluralRules(pluralRules)) === null) {
        return callFunction(CallPluralRulesMethodIfWrapped, this, value,
                            "Intl_PluralRules_select");
    }
    let n = ToNumber(value);
    getPluralRulesInternals(pluralRules);
    return intl_SelectPluralRule(pluralRules, n);
}
function Intl_PluralRules_resolvedOptions() {
    var pluralRules = this;
    if (!IsObject(pluralRules) || (pluralRules = GuardToPluralRules(pluralRules)) === null) {
        return callFunction(CallPluralRulesMethodIfWrapped, this,
                            "Intl_PluralRules_resolvedOptions");
    }
    var internals = getPluralRulesInternals(pluralRules);
    var result = {
        locale: internals.locale,
        type: internals.type,
        minimumIntegerDigits: internals.minimumIntegerDigits,
    };
    ;;
    if (hasOwn("minimumFractionDigits", internals)) {
        _DefineDataProperty(result, "minimumFractionDigits", internals.minimumFractionDigits);
        _DefineDataProperty(result, "maximumFractionDigits", internals.maximumFractionDigits);
    }
    ;;
    if (hasOwn("minimumSignificantDigits", internals)) {
        _DefineDataProperty(result, "minimumSignificantDigits",
                            internals.minimumSignificantDigits);
        _DefineDataProperty(result, "maximumSignificantDigits",
                            internals.maximumSignificantDigits);
    }
    var internalsPluralCategories = internals.pluralCategories;
    if (internalsPluralCategories === null) {
        internalsPluralCategories = intl_GetPluralCategories(pluralRules);
        internals.pluralCategories = internalsPluralCategories;
    }
    var pluralCategories = [];
    for (var i = 0; i < internalsPluralCategories.length; i++)
        _DefineDataProperty(pluralCategories, i, internalsPluralCategories[i]);
    _DefineDataProperty(result, "pluralCategories", pluralCategories);
    return result;
}
var relativeTimeFormatInternalProperties = {
    localeData: relativeTimeFormatLocaleData,
    relevantExtensionKeys: ["nu"],
};
function relativeTimeFormatLocaleData() {
    return {
        nu: getNumberingSystems,
        default: {
            nu: intl_numberingSystem,
        },
    };
}
function resolveRelativeTimeFormatInternals(lazyRelativeTimeFormatData) {
    ;;
    var internalProps = std_Object_create(null);
    var RelativeTimeFormat = relativeTimeFormatInternalProperties;
    const r = ResolveLocale("RelativeTimeFormat",
                            lazyRelativeTimeFormatData.requestedLocales,
                            lazyRelativeTimeFormatData.opt,
                            RelativeTimeFormat.relevantExtensionKeys,
                            RelativeTimeFormat.localeData);
    internalProps.locale = r.locale;
    internalProps.numberingSystem = r.nu;
    internalProps.style = lazyRelativeTimeFormatData.style;
    internalProps.numeric = lazyRelativeTimeFormatData.numeric;
    return internalProps;
}
function getRelativeTimeFormatInternals(obj) {
    ;;
    ;;
    var internals = getIntlObjectInternals(obj);
    ;;
    var internalProps = maybeInternalProperties(internals);
    if (internalProps)
        return internalProps;
    internalProps = resolveRelativeTimeFormatInternals(internals.lazyData);
    setInternalProperties(internals, internalProps);
    return internalProps;
}
function InitializeRelativeTimeFormat(relativeTimeFormat, locales, options) {
    ;;
    ;;
    const lazyRelativeTimeFormatData = std_Object_create(null);
    let requestedLocales = CanonicalizeLocaleList(locales);
    lazyRelativeTimeFormatData.requestedLocales = requestedLocales;
    if (options === undefined)
        options = std_Object_create(null);
    else
        options = ToObject(options);
    let opt = new Record();
    let matcher = GetOption(options, "localeMatcher", "string", ["lookup", "best fit"], "best fit");
    opt.localeMatcher = matcher;
    let numberingSystem = GetOption(options, "numberingSystem", "string", undefined, undefined);
    if (numberingSystem !== undefined) {
        numberingSystem = intl_ValidateAndCanonicalizeUnicodeExtensionType(numberingSystem,
                                                                           "numberingSystem",
                                                                           "nu");
    }
    opt.nu = numberingSystem;
    lazyRelativeTimeFormatData.opt = opt;
    const style = GetOption(options, "style", "string", ["long", "short", "narrow"], "long");
    lazyRelativeTimeFormatData.style = style;
    const numeric = GetOption(options, "numeric", "string", ["always", "auto"], "always");
    lazyRelativeTimeFormatData.numeric = numeric;
    initializeIntlObject(relativeTimeFormat, "RelativeTimeFormat", lazyRelativeTimeFormatData);
}
function Intl_RelativeTimeFormat_supportedLocalesOf(locales ) {
    var options = arguments.length > 1 ? arguments[1] : undefined;
    var availableLocales = "RelativeTimeFormat";
    let requestedLocales = CanonicalizeLocaleList(locales);
    return SupportedLocales(availableLocales, requestedLocales, options);
}
function Intl_RelativeTimeFormat_format(value, unit) {
    let relativeTimeFormat = this;
    if (!IsObject(relativeTimeFormat) ||
        (relativeTimeFormat = GuardToRelativeTimeFormat(relativeTimeFormat)) === null)
    {
        return callFunction(CallRelativeTimeFormatMethodIfWrapped, this, value, unit,
                            "Intl_RelativeTimeFormat_format");
    }
    let t = ToNumber(value);
    let u = ToString(unit);
    var internals = getRelativeTimeFormatInternals(relativeTimeFormat);
    return intl_FormatRelativeTime(relativeTimeFormat, t, u, internals.numeric,
                                   false);
}
function Intl_RelativeTimeFormat_formatToParts(value, unit) {
    let relativeTimeFormat = this;
    if (!IsObject(relativeTimeFormat) ||
        (relativeTimeFormat = GuardToRelativeTimeFormat(relativeTimeFormat)) === null)
    {
        return callFunction(CallRelativeTimeFormatMethodIfWrapped, this, value, unit,
                            "Intl_RelativeTimeFormat_formatToParts");
    }
    let t = ToNumber(value);
    let u = ToString(unit);
    var internals = getRelativeTimeFormatInternals(relativeTimeFormat);
    return intl_FormatRelativeTime(relativeTimeFormat, t, u, internals.numeric,
                                   true);
}
function Intl_RelativeTimeFormat_resolvedOptions() {
    var relativeTimeFormat = this;
    if (!IsObject(relativeTimeFormat) ||
        (relativeTimeFormat = GuardToRelativeTimeFormat(relativeTimeFormat)) === null)
    {
        return callFunction(CallRelativeTimeFormatMethodIfWrapped, this,
                            "Intl_RelativeTimeFormat_resolvedOptions");
    }
    var internals = getRelativeTimeFormatInternals(relativeTimeFormat);
    var result = {
        locale: internals.locale,
        style: internals.style,
        numeric: internals.numeric,
        numberingSystem: internals.numberingSystem,
    };
    return result;
}
var sanctionedSimpleUnitIdentifiers = {
    "acre": true,
    "bit": true,
    "byte": true,
    "celsius": true,
    "centimeter": true,
    "day": true,
    "degree": true,
    "fahrenheit": true,
    "fluid-ounce": true,
    "foot": true,
    "gallon": true,
    "gigabit": true,
    "gigabyte": true,
    "gram": true,
    "hectare": true,
    "hour": true,
    "inch": true,
    "kilobit": true,
    "kilobyte": true,
    "kilogram": true,
    "kilometer": true,
    "liter": true,
    "megabit": true,
    "megabyte": true,
    "meter": true,
    "mile": true,
    "mile-scandinavian": true,
    "milliliter": true,
    "millimeter": true,
    "millisecond": true,
    "minute": true,
    "month": true,
    "ounce": true,
    "percent": true,
    "petabyte": true,
    "pound": true,
    "second": true,
    "stone": true,
    "terabit": true,
    "terabyte": true,
    "week": true,
    "yard": true,
    "year": true
};