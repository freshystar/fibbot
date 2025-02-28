## FibBot Github Action
#### Overview

Develop a GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action will support two parameters (e.g., a flag to enable Fibonacci calculation and a threshold limit).

Hereby, is an overview of the diffirent actions that were implemented;

### Hello World docker action 

This action prints "Hello World" to the `action.log`.

#### Example usage

```
uses: actions/fibbot@v1
```
In my case, I used:
```
 - name: Run Rust Action
        uses: ./  # Points to your action
        with:
```


### Parameter parsing
 After updating my `action.yml` file, I can now receive and process inputs from the Github worflow:
```
- name: Run Rust Action
        uses: ./  # Points to your action
        with:
          enable_fib: "true"
          max_threshold: "50"
```

### Logic  Developmant
A robust fibonacci function with tests


