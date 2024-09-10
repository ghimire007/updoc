use crate::crdt::working::chunk::{Node, T};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug)]
pub struct SkipNode {
    parent: Option<Weak<RefCell<SkipNode>>>,
    child: Option<Rc<RefCell<SkipNode>>>,
    prev: Option<Weak<RefCell<SkipNode>>>,
    next: Option<Rc<RefCell<SkipNode>>>,
    content: Option<Rc<RefCell<Node<T>>>>,
    end: u32,
    level: u32,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct SkipListIndex {

    first: Option<Rc<RefCell<SkipNode>>>,
    delimeter: usize,
    last: Option<Rc<RefCell<SkipNode>>>,

}

impl SkipNode {
    pub fn new(end: u32, level: u32, index: usize) -> Self {
        println!("{:?}  {:?}", level, index);
        Self {
            end,
            level,
            index,
            parent: None,
            child: None,
            prev:None,
            next: None,
            content: None,
        }
    }
}

impl<'a> SkipListIndex {

    pub fn new(end: u32, level: u32, index: usize, delimeter: usize) -> Self {
        //let sn= SkipNode::new(end,level,index);
        //Rc::new(RefCell::new(sn.clone())),
        Self {
            first: None,
            delimeter,
            last: None,
        }
    }

    pub fn insert_node(&mut self, end: u32, content: &Node<T>) {
        let  ln =self.last.clone();
        if let Some(ref mut last) = self.last {
            let l = last.clone();
            let mut last_node = l.borrow_mut();
           

            //if level capacity has not max out
            if last_node.index < self.delimeter - 1 {
                //println!("{:?}", last_node.index);
                let mut new_node = SkipNode::new(end, last_node.level, last_node.index + 1);

                new_node.content = Some(Rc::new(RefCell::new(content.clone())));
                new_node.parent = last_node.parent.clone();
                let binding= Rc::new(RefCell::new(last_node.clone()));
                new_node.prev=Some(Rc::downgrade(&binding));
                last_node.next = Some(Rc::new(RefCell::new(new_node.clone())));
                self.last = Some(Rc::new(RefCell::new(new_node)));
            } else {
                // add new level
                let mut new_node = SkipNode::new(end, 0, 0);

                new_node.content = Some(Rc::new(RefCell::new(content.clone())));
                let mut prev_parent = last_node.parent.clone();
                //get self parent

                self.insert_parent(&mut last_node.parent, &mut new_node,&ln.clone());

                let binding= Rc::new(RefCell::new(last_node.clone()));
                new_node.prev=Some(Rc::downgrade(&binding));

                last_node.next = Some(Rc::new(RefCell::new(new_node.clone())));
                self.last = Some(Rc::new(RefCell::new(new_node)));
            }
        } else {
            // initialize the  skipped list if empty
            let mut new_node = SkipNode::new(end, 0, 0);
            new_node.content = Some(Rc::new(RefCell::new(content.clone())));
            self.first = Some(Rc::new(RefCell::new(new_node.clone())));
            self.last = Some(Rc::new(RefCell::new(new_node.clone())));

            println!("skipped list first node inserted");
        }
    }

    fn insert_parent(
        &mut self,
        prev_parent: &mut Option<Weak<RefCell<SkipNode>>>,
        current_node: &mut SkipNode,
        prev_child:&Option<Rc<RefCell<SkipNode>>>
    ) {
        println!("insert parent called");

        if let Some(prev_parent) = prev_parent {
            if let Some(strong_parent) = prev_parent.upgrade() {
                let mut sp = strong_parent.borrow_mut();
                if (sp.index < self.delimeter - 1) {
                    let mut new_parent = SkipNode::new(current_node.end, sp.level, sp.index + 1);
                    new_parent.parent = sp.parent.clone();

                    //set child of new parent
                    new_parent.child = Some(Rc::new(RefCell::new(current_node.clone())));

                    let binding= Rc::new(RefCell::new(sp.clone()));
                    new_parent.prev=Some(Rc::downgrade(&binding));
                    sp.next = Some(Rc::new(RefCell::new(new_parent.clone())));

                    //set childs parent
                    current_node.parent =
                        Some(Rc::downgrade(&Rc::new(RefCell::new(new_parent.clone()))));
                } else {
                    let mut new_parent = SkipNode::new(current_node.end, sp.level, 0);

                    //let mut new_node = SkipNode::new(end, 0, 0);
                    let mut prev_parent = sp.parent.clone();
                    self.insert_parent(&mut sp.parent, &mut new_parent,prev_child);

                    let binding= Rc::new(RefCell::new(sp.clone()));
                    new_parent.prev=Some(Rc::downgrade(&binding));
                    sp.next = Some(Rc::new(RefCell::new(new_parent.clone())));
                }
            } else {

                //handle this case
            }
        } else {//

            //your previous friend in also in requirement of parent
            let root = self.first.clone();
            if let Some(first) = root {
                let mut new_parent = SkipNode::new(current_node.end, first.borrow().level + 1, 1);
                new_parent.child = Some(Rc::new(RefCell::new(current_node.clone())));
                current_node.parent =Some(Rc::downgrade(&Rc::new(RefCell::new(new_parent.clone()))));
                self.create_parent_for_friend(&mut new_parent, prev_child)

                //self.first = Some(Rc::new(RefCell::new(new_parent.clone())));
            }
        }

    }




    fn create_parent_for_friend(&mut self,new_parent:&mut SkipNode,friend:&Option<Rc<RefCell<SkipNode>>> ){
        //let mut cur_child=friend.clone();

        println!("reached here");

        if let Some( d) = friend.as_ref().cloned(){
           let mut dost=d.borrow_mut();
            let mut dost_father = SkipNode::new(dost.end, new_parent.level, 0);
            //new_parent.child = Some(Rc::new(RefCell::new(dost.clone())));
            dost_father.next=Some(Rc::new(RefCell::new(new_parent.clone())));
            let fat_ref=Rc::new(RefCell::new(dost_father));
            dost.parent =Some(Rc::downgrade(&fat_ref));
            new_parent.prev=Some(Rc::downgrade(&fat_ref));
            let mut i=dost.index-1;
            let mut cur_child=dost.prev.clone();
            while i!=0{
                let mut x= cur_child.unwrap().upgrade().unwrap();
                x.borrow_mut().parent=Some(Rc::downgrade(&fat_ref));
                cur_child=x.borrow().prev.clone();
            }
            let mut x= cur_child.unwrap().upgrade().unwrap();
            x.borrow_mut().parent=Some(Rc::downgrade(&fat_ref));
            self.first = Some(Rc::clone(&fat_ref));

        }



    }




    pub fn print_index(&self,node:&Option<Rc<RefCell<SkipNode>>>,next_child:&Option<Rc<RefCell<SkipNode>>>){
        if let Some(ref cur)=node{
            let c=cur.borrow();
            print!("{:?}  {:?}  {:?}        ",c.level,c.index,c.end);
            if c.next.is_some(){
                println!("goingin to nextt ----->");
                self.print_index(&c.next,next_child);

            }else if c.child.is_some(){
                println!("goingin to child ");
                println!("");

                let mut x= next_child.clone();
                x= x.unwrap().borrow().child.clone();
                self.print_index(next_child,&x)

            }
            else{

            }


        }
    }
    
    // fn pi(&self,node:&Option<Rc<RefCell<SkipNode>>>,cur_levl:i32){
    //     if let Some(ref cur)=node{
    //         let c=cur.borrow();
    //         if c.level 
            

    //     }
    // }



    pub fn search(&self,node:&'a Option<Rc<RefCell<SkipNode>>>, index:u32)->Option <Rc<RefCell<Node<T>>>>{
        if let Some(ref cur)=node{
            let c=cur.borrow_mut();
            println!("{:?} end for this node ",c.end);
            if(index<c.end){
                println!("value is small going to check for content ");

                if let Some(ref content)= c.content{
                  return Some(content.clone());
         
                }else{
                    println!("content not found . going to child");
                    if c.child.is_some(){
                    return self.search(&c.child,index);
                    }
                    else{
                        return None;
                    }
                }

            }
            else{
                println!("going to next node ");
                if c.next.is_some(){
                
                return self.search(&c.next,index);
                }else{
                    return None;
                }
            }



        }
        None


    }










}

#[test]
fn test_insert() {

    let mut skip_list_index = SkipListIndex::new(0, 0, 0, 5);
    let content1: Node<T> = Node::new("hellowor00".to_owned(), (0, 0, 0), false, 0, 10);
    let content2: Node<T> = Node::new("hellowor01".to_owned(), (0, 0, 1), false, 10, 10);
    let content3: Node<T> = Node::new("hellowor02".to_owned(), (0, 0, 2), false, 20, 10);
    let content4: Node<T> = Node::new("hellowor03".to_owned(), (0, 0, 3), false, 30, 10);
    let content5: Node<T> = Node::new("hellowor04".to_owned(), (0, 0, 4), false, 40, 10);
    let content6: Node<T> = Node::new("hellowor05".to_owned(), (0, 0, 5), false, 50, 10);
    let content7: Node<T> = Node::new("hellowor06".to_owned(), (0, 0, 6), false, 60, 10);
    let content8: Node<T> = Node::new("hellowor07".to_owned(), (0, 0, 7), false, 70, 10);
    let content9: Node<T> = Node::new("hellowor08".to_owned(), (0, 0, 8), false, 80, 10);
    let content10: Node<T> = Node::new("hellowor09".to_owned(), (0, 0, 9), false, 90, 10);
    
    let content11: Node<T> = Node::new("hellowor10".to_owned(), (0, 0, 10), false, 100, 10);
    let content12: Node<T> = Node::new("hellowor11".to_owned(), (0, 0, 11), false, 110, 10);
    let content13: Node<T> = Node::new("hellowor12".to_owned(), (0, 0, 12), false, 120, 10);
    let content14: Node<T> = Node::new("hellowor13".to_owned(), (0, 0, 13), false, 130, 10);
    let content15: Node<T> = Node::new("hellowor14".to_owned(), (0, 0, 14), false, 140, 10);
    let content16: Node<T> = Node::new("hellowor15".to_owned(), (0, 0, 15), false, 150, 10);
    let content17: Node<T> = Node::new("hellowor16".to_owned(), (0, 0, 16), false, 160, 10);
    let content18: Node<T> = Node::new("hellowor17".to_owned(), (0, 0, 17), false, 170, 10);
    let content19: Node<T> = Node::new("hellowor18".to_owned(), (0, 0, 18), false, 180, 10);
    let content20: Node<T> = Node::new("hellowor19".to_owned(), (0, 0, 19), false, 190, 10);
    
    let content21: Node<T> = Node::new("hellowor20".to_owned(), (0, 0, 20), false, 200, 10);
    let content22: Node<T> = Node::new("hellowor21".to_owned(), (0, 0, 21), false, 210, 10);
    let content23: Node<T> = Node::new("hellowor22".to_owned(), (0, 0, 22), false, 220, 10);
    
  

    skip_list_index.insert_node(10, &content1);
    skip_list_index.insert_node(20, &content2);
    skip_list_index.insert_node(30, &content3);
    skip_list_index.insert_node(40, &content4);
    skip_list_index.insert_node(50, &content5);
    skip_list_index.insert_node(60, &content6);
    skip_list_index.insert_node(70, &content7);
    skip_list_index.insert_node(80, &content8);
    skip_list_index.insert_node(90, &content9);
    skip_list_index.insert_node(100, &content10);
    skip_list_index.insert_node(110, &content11);
    skip_list_index.insert_node(120, &content12);
    skip_list_index.insert_node(130, &content13);
    skip_list_index.insert_node(140, &content14);
    skip_list_index.insert_node(150, &content15);
    skip_list_index.insert_node(160, &content16);
    skip_list_index.insert_node(170, &content17);
    skip_list_index.insert_node(180, &content18);
    skip_list_index.insert_node(190, &content19);
    skip_list_index.insert_node(200, &content20);
    skip_list_index.insert_node(210, &content21);
    skip_list_index.insert_node(200, &content22);
    skip_list_index.insert_node(230, &content23);
   
   let mut c= skip_list_index.first.clone();
   c=c.unwrap().borrow().child.clone();
   skip_list_index.print_index(&skip_list_index.first,&c );

    //println!("{:?}",skip_list_index.search(&skip_list_index.first, 22));
}
   