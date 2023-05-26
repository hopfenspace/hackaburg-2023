import React from "react";

export type InputProps = {
    value: string;
    onChange: (newValue: string) => any;
} & Omit<React.InputHTMLAttributes<HTMLInputElement>, "value" | "onChange">;

const Input = React.forwardRef(function Input(props: InputProps, ref: React.ForwardedRef<HTMLInputElement>) {
    const { value, onChange, ...passThrough } = props;

    return (
        <input
            className="input"
            value={value}
            onChange={(e) => {
                onChange(e.target.value);
            }}
            ref={ref}
            {...passThrough}
        />
    );
});
export default Input;
