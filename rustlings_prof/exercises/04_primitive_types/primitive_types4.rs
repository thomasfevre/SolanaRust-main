fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        //let deuxieme = 1;
        //let avant_dernier = a.len() - 1;
        //let nice_slice = &a[deuxieme..avant_dernier];
        let nice_slice = &a[1..4];
        //let nice_slice = [a[1],a[2],a[3]];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
