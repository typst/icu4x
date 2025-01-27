// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An iterator over code point ranges, produced by `CodePointSetData` or
/// one of the `CodePointMapData` types
final class CodePointRangeIterator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;
  final core.List<Object> _edge_a;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  CodePointRangeIterator._(this._underlying, bool isOwned, this._edge_self, this._edge_a) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_CodePointRangeIterator_destroy));

  /// Advance the iterator by one and return the next range.
  ///
  /// If the iterator is out of items, `done` will be true
  CodePointRangeIteratorResult next() {
    final result = _CodePointRangeIterator_next(_underlying);
    return CodePointRangeIteratorResult._(result);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'CodePointRangeIterator_destroy')
// ignore: non_constant_identifier_names
external void _CodePointRangeIterator_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_CodePointRangeIteratorResultFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'CodePointRangeIterator_next')
// ignore: non_constant_identifier_names
external _CodePointRangeIteratorResultFfi _CodePointRangeIterator_next(ffi.Pointer<ffi.Opaque> self);
