#![allow(unused)]
mod data;
mod parser;

#[cfg(test)]
mod tests {
    use crate::data::*;
    use crate::parser::*;
    use regex::Regex;

    #[test]
    fn test_format_data() {
        let re =
            Regex::new(r"(?m)([hral]{1,2}[ej][1-7])[^\d+-]{1,6}([+-]?\d*\.?\d*[Ee+-]{0,2}\d+)")
                .unwrap();

        let msg1 = vec![
            "hj1 86.79",
            "hj2 -45.00",
            "raj1 -81.43",
            "raj2 -14.65",
            "raj3 70.00",
            "raj4 15.00",
            "laj1 -81.43",
            "laj2 15.35",
            "laj3 -70.00",
            "laj4 -15.00",
            "rlj1 -0.16",
            "rlj2 1.26",
            "rlj3 16.11",
            "rlj4 -66.69",
            "rlj5 47.56",
            "rlj6 -1.27",
            "rlj7 0.00",
            "llj1 -0.16",
            "llj2 -0.43",
            "llj3 12.71",
            "llj4 -56.48",
            "llj5 43.36",
            "(FRP (n lf",
            " (c -0.03 -0.03 -0.01",
            " (f 30.72 -3.92 94.58",
            "llj6 0.40",
            "(FRP (n lf1",
            " (c -0.03 -0.01 -0.01",
            " (f 0.50 -7.37 39.59",
            "llj7 -0.00",
            "",
        ];
        let msg2 = "(time (now 118.81))(GS (sl 0) (sr 0) (t 4.62) (pm KickOff_Left))(GYR (n torso) (rt -79.44 10.53 -26.89))(ACC (n torso) \
        (a -0.37 -0.31 -0.20))(HJ (n hj1) (ax 86.79))(HJ (n hj2) (ax -45.00))(HJ (n raj1) (ax -81.43))(HJ (n raj2) (ax -14.65))    )(HJ (n raj3) \
        (ax 70.00))(HJ (n raj4) (ax 15.00))(HJ (n laj1) (ax -81.43))(HJ (n laj2) (ax 15.35))(HJ (n laj3) (ax -70.00))(HJ (n laj4) (ax -15.00))(HJ \
            (n rlj1) (ax -0.16))(HJ (n rlj2) (ax 1.26))(HJ (n rlj3) (ax 16.11))(HJ (n rlj4) (ax -66.69))(HJ (n rlj5) (ax 47.56))(HJ (n rlj6) \
            (ax -1.27))(HJ (n rlj7) (ax 0.00))(HJ (n llj1) (ax -0.16))(HJ (n llj2) (ax -0.43))(HJ (n llj3) (ax 12.71))(HJ (n llj4) (ax -56.48))(HJ \
                (n llj5) (ax 43.36))(FRP (n lf) (c -0.03 -0.03 -0.01) (f 30.72 -3.92 94.58))(HJ (n llj6) (ax 0.40))(FRP (n lf1) (c -0.03 -0.01 -0.01) \
                (f 0.50 -7.37 39.59))(HJ (n llj7) (ax -0.00))";

        let res1 = format_data(msg1);
        let res2 = format_data_regex(&msg2, &re);

        assert_eq!(res1, res2);
    }

    #[test]
    fn test_get_joint_idx() {
        assert_eq!(get_joint_idx("la1"), 0);
    }

    #[test]
    fn test_parser() {
        let (sensor_values1, mut speeds1) = parser("assets/kuaiti.pcapng");
        let (sensor_values2, mut speeds2) = parser_regex("assets/kuaiti.pcapng");
        assert_eq!(sensor_values1, sensor_values1);
        assert_eq!(speeds1, speeds2);
    }
}
