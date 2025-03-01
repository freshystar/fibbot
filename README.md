
[![Fibonacci](https://github.com/freshystar/fibbot/actions/workflows/build.yml/badge.svg)](https://github.com/freshystar/fibbot/actions/workflows/build.yml)


## FibBot Github Action
#### Overview

Develop a GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action will support two parameters (e.g., a flag to enable Fibonacci calculation and a threshold limit).

Hereby, is an overview of the diffirent actions that were implemented;

### Hello World docker action 

This action prints "Hello World" to the `action.log`.

#### Example usage

```rs
uses: actions/fibbot@v1
```
In my case, I used:
```rs
 - name: Run Rust Action
        uses: ./ # Points to your action
        with:
```


### Parameter parsing
 After updating my `action.yml` file, I can now receive and process inputs from the Github worflow:
```rs
- name: Run Rust Action
        uses: ./  # Points to your action
        with:
          enable_fib: "true"
          max_threshold: "50"
```

### Logic  Development

In this repository, you will find a file `fibbonacci.rs` under the `src` directory which contains a robust **fibonacci** function.
Here's a brief overview of the logic:
```rs
 while i < num {
            let temp = b;
            b = &temp + a;
            a = temp;
            i += 1;
 }
 ```

### Full Workflow Testing

After several updates and implementations, we now have an action that can be successfully executed on pull requests, extract numbers and return their fibonacci with a comment.



