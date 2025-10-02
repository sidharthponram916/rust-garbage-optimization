use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

use crate::types::{Memory, RefCountMem};
use crate::utils::*;

pub fn reference_counting(string_vec: Vec<String>) ->  RefCountMem {
    let re = Regex::new(r"^(?P<cmd_heap>Ref Heap) (?P<heap>\d+(?: \d+)*)$|^(?P<cmd_stack>Ref Stack)(?P<stack>(?: \d+)*)$|^(?P<cmd_pop>Pop)$").unwrap();
    let mut stack: Vec<Vec<u32>> = vec![]; 
    let mut heap: Vec<(Option<Vec<u32>>, u32)> = vec![(None, 0); 10];
            
    for s in string_vec.iter() { 
        let s = s.trim(); 

        if let Some(caps) = re.captures(s) { 

            if let Some(cmd) = caps.name("cmd_stack") { 
                if let Some(nums) = caps.name("stack") { 
                    let values: Vec<u32> = nums.as_str()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(); 

                    for num in &values { 
                        let index = (num - 0) as usize;
                        heap[index].0 = Some(vec![]);
                        heap[index].1 += 1;
                    }
                    
                    stack.push(values); 
                }
            }
            else if let Some(cmd) = caps.name("cmd_heap") { 
                if let Some(nums) = caps.name("heap") { 
                    let values: Vec<u32> = nums.as_str()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(); 
                    
                    let root = (values[0] - 0) as usize; 

                    if !heap[root].0.is_none() {
                        heap[root].0 = Some(values[1..].to_vec());

                        for num in &values[1..] { 
                            let index = (num - 0) as usize;

                            if heap[index].0.is_none() {
                                heap[index].0 = Some(vec![]);
                            }

                            heap[index].1 += 1;
                        }
                    }
                }
            }
            else if let Some(_cmd) = caps.name("cmd_pop") {
                let mut queue = vec![]; 
            
                if let Some(frame) = stack.pop() {
                    for &index in &frame {
                        queue.push(index); 
                    }
                    
                    while (!queue.is_empty()) { 
                            let val = (queue.remove(0) - 0) as usize; 
                            
                            heap[val].1 = heap[val].1 - 1; 
                            
                            if (heap[val].1 == 0) { 
                                if let Some(children) = heap[val].0.take() {
                                    for &child in &children { 
                                        queue.push(child); 
                                    }
                                }
                            }
                    }
                }
            }
        }
    }

    RefCountMem { 
        stack: stack, 
        heap: heap
    }
}

// suggested helper function. You may modify parameters as you wish.
// Takes in some form of stack and heap and returns all indicies in heap
// that can be reached.
pub fn reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32> {
    let mut reachable_array = vec![];
    let mut visited = vec![]; 

    for s in stack {
        let mut queue = vec![];

        for &index in s {
            if (!visited.contains(&index)) { 
                queue.push(index);
                visited.push(index); 
                reachable_array.push(index);
            }
        }

        while !queue.is_empty() {
            let val = queue.remove(0) as usize;

            if let Some((_, children)) = &heap[val] {
                for &child in children {
                    if (!visited.contains(&child)) { 
                     queue.push(child);
                     visited.push(child); 
                     reachable_array.push(child);
                    }   
                }
            }
        }
    }

    reachable_array
}


pub fn mark_and_sweep(mem: &mut Memory) -> () {
    let mut reachable_array = reachable(&mem.stack, &mem.heap); 
    
    let mut index = 0; 
    for entry in mem.heap.iter_mut() {
        if !reachable_array.contains(&index) {
            *entry = None; 
        }
    
        index = index + 1; 
    }
}

// alive says which half is CURRENTLY alive. You must copy to the other half.
// 0 for left side currently in use, 1 for right side currently in use
pub fn stop_and_copy(mem: &mut Memory, alive: u32) {
    let mut reachable_array = reachable(&mem.stack, &mem.heap); 
    let original_length = (mem.heap.len() as u32) / 2; 
        if (alive == 0) { 
            let mut index = mem.heap.len() / 2;

            let mut to_copy: Vec<(usize, Option<(String, Vec<u32>)>)> = vec![];
        
            for entry in &mut mem.heap[index..] {
                *entry = None;
            }

            for (i, entry) in mem.heap.iter().enumerate().take(index) {
                if reachable_array.contains(&(i as u32)) {
                    if let Some(e) = entry { 
                        let mut adjusted_children = e.1.clone();
                        for ele in &mut adjusted_children {
                            *ele += original_length;
                        }
                    
                        to_copy.push((index, Some((e.0.clone(), adjusted_children))));
                        index += 1;
                    }
                }
            }

            for (dest_index, value) in to_copy {
                mem.heap[dest_index] = value;
            }
            
        for frame in &mut mem.stack {
            for ele in frame.iter_mut() { 
                if let Some((ref target_letter, _)) = mem.heap[*ele as usize] {
                    for (i, entry) in mem.heap.iter().enumerate().skip(*ele as usize + 1) {
                        if let Some((letter, _)) = entry {
                            if letter == target_letter {
                                *ele = i as u32; 
                                break; 
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        let mut og_index = mem.heap.len() / 2; 
        let mut index = 0; 
        let mut to_copy: Vec<(usize, Option<(String, Vec<u32>)>)> = vec![];

        
        for entry in &mut mem.heap[..og_index] {
            *entry = None;
        }   
        
        for (i, entry) in mem.heap.iter().enumerate().skip(og_index) {
            if reachable_array.contains(&(i as u32)) {
                if let Some(e) = entry {
                    let mut adjusted_children = e.1.clone();
        
                    for ele in &mut adjusted_children {
                        *ele -= original_length; 
                    }
        
                    to_copy.push((index, Some((e.0.clone(), adjusted_children))));
                    index += 1;
                }
            }
        }

        for (dest_index, value) in to_copy {
            mem.heap[dest_index] = value;
        }
        
        for frame in &mut mem.stack {
            for ele in frame.iter_mut() {
                if let Some((ref target_letter, _)) = mem.heap[*ele as usize] {
                    for (i, entry) in mem.heap[..*ele as usize].iter().enumerate().rev() {
                        if let Some((letter, _)) = entry {
                            if letter == target_letter {
                                *ele = i as u32; 
                                break; 
                            }
                        }
                    }
                }
            }
        }
    }
}
