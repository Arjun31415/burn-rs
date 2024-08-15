// Language
use alloc::vec;
use alloc::vec::Vec;
use burn_tensor::ops::{BoolTensorOps, IntTensorOps};
use burn_tensor::ElementConversion;
use core::ops::Range;
use ndarray::{IntoDimension, Zip};

// Current crate
use crate::element::{FloatNdArrayElement, QuantElement};
use crate::NdArrayDevice;
use crate::{tensor::NdArrayTensor, NdArray};

// Workspace crates
use burn_tensor::{backend::Backend, Shape, TensorData};

use super::NdArrayOps;

impl<E: FloatNdArrayElement, Q: QuantElement> BoolTensorOps<Self> for NdArray<E, Q> {
    fn bool_from_data<const D: usize>(
        data: TensorData,
        _device: &NdArrayDevice,
    ) -> NdArrayTensor<bool, D> {
        NdArrayTensor::from_data(data)
    }

    fn bool_shape<const D: usize>(
        tensor: &<NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> Shape<D> {
        tensor.shape()
    }

    async fn bool_into_data<const D: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> TensorData {
        let shape = tensor.shape();
        let values = tensor.array.into_iter().collect();
        TensorData::new(values, shape)
    }

    fn bool_to_device<const D: usize>(
        tensor: NdArrayTensor<bool, D>,
        _device: &NdArrayDevice,
    ) -> NdArrayTensor<bool, D> {
        tensor
    }

    fn bool_reshape<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<bool, D1>,
        shape: Shape<D2>,
    ) -> NdArrayTensor<bool, D2> {
        NdArrayOps::reshape(tensor, shape)
    }

    fn bool_slice<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<bool, D1>,
        ranges: [Range<usize>; D2],
    ) -> NdArrayTensor<bool, D1> {
        NdArrayOps::slice(tensor, ranges)
    }

    fn bool_into_int<const D: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> NdArrayTensor<i64, D> {
        let shape = tensor.shape();
        let values = tensor.array.into_iter().collect();
        NdArray::<E>::int_from_data(
            TensorData::new(values, shape).convert::<i64>(),
            &NdArrayDevice::Cpu,
        )
    }

    fn bool_device<const D: usize>(
        _tensor: &<NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> <NdArray<E> as Backend>::Device {
        NdArrayDevice::Cpu
    }

    fn bool_empty<const D: usize>(
        shape: Shape<D>,
        _device: &<NdArray<E> as Backend>::Device,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D> {
        let values = vec![false; shape.num_elements()];
        NdArrayTensor::from_data(TensorData::new(values, shape))
    }

    fn bool_slice_assign<const D1: usize, const D2: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D1>,
        ranges: [Range<usize>; D2],
        value: <NdArray<E> as Backend>::BoolTensorPrimitive<D1>,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D1> {
        NdArrayOps::slice_assign(tensor, ranges, value)
    }

    fn bool_cat<const D: usize>(
        tensors: Vec<<NdArray<E> as Backend>::BoolTensorPrimitive<D>>,
        dim: usize,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D> {
        NdArrayOps::cat(tensors, dim)
    }

    fn bool_equal<const D: usize>(
        lhs: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
        rhs: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D> {
        let output = Zip::from(&lhs.array)
            .and(&rhs.array)
            .map_collect(|&lhs_val, &rhs_val| (lhs_val == rhs_val))
            .into_shared();
        NdArrayTensor::new(output)
    }

    fn bool_not<const D: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D> {
        let array = tensor.array.mapv(|a| !a).into_shared();
        NdArrayTensor { array }
    }

    fn bool_into_float<const D: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
    ) -> <NdArray<E> as Backend>::FloatTensorPrimitive<D> {
        let array = tensor.array.mapv(|a| (a as i32).elem()).into_shared();
        NdArrayTensor { array }
    }

    fn bool_swap_dims<const D: usize>(
        tensor: <NdArray<E> as Backend>::BoolTensorPrimitive<D>,
        dim1: usize,
        dim2: usize,
    ) -> <NdArray<E> as Backend>::BoolTensorPrimitive<D> {
        NdArrayOps::swap_dims(tensor, dim1, dim2)
    }

    fn bool_permute<const D: usize>(
        tensor: burn_tensor::ops::BoolTensor<Self, D>,
        axes: [usize; D],
    ) -> burn_tensor::ops::BoolTensor<Self, D> {
        let array = tensor.array.permuted_axes(axes.into_dimension());
        NdArrayTensor { array }
    }

    fn bool_expand<const D1: usize, const D2: usize>(
        tensor: burn_tensor::ops::BoolTensor<Self, D1>,
        shape: Shape<D2>,
    ) -> burn_tensor::ops::BoolTensor<Self, D2> {
        NdArrayOps::expand(tensor, shape)
    }

    fn bool_flip<const D: usize>(
        tensor: burn_tensor::ops::BoolTensor<Self, D>,
        axes: &[usize],
    ) -> burn_tensor::ops::BoolTensor<Self, D> {
        NdArrayOps::flip(tensor, axes)
    }
}
