#[macro_use]
extern crate microstate;

microstate!{
    MicroMachine { New }
    states { New, Confirmed, Ignored }

    confirm {
        New => Confirmed
    }

    ignore {
        New => Ignored
    }

    reset {
        Confirmed => New
        Ignored   => New
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ManyStates {
    New,
    Confirmed,
    Ignored,
    Panic,
    Suplex
}

microstate_ext! {
    MicroFoo { New, ManyStates }

    confirm {
        New => Confirmed
    }

    ignore {
        New => Ignored
    }

    reset {
        Confirmed => New
        Ignored   => New
    }
}

microstate_ext! {
    MicroBar { New, ManyStates }

    confirm {
        New => Confirmed
    }

    ignore {
        New => Ignored
    }

    reset {
        Confirmed => New
        Ignored   => New
    }

    panic {
        Confirmed => Panic
        Ignored => Panic
        New => Panic
    }

    suplex {
        Panic => Suplex
    }
}

fn main() {
    let mut machine = MicroMachine::new();

    println!("{:?}", machine.confirm());
    println!("{:?}", machine.state());

    println!("{:?}", machine.ignore());
    println!("{:?}", machine.state());

    println!("{:?}", machine.reset());
    println!("{:?}", machine.state());

    println!("{:?}", machine.ignore());
    println!("{:?}", machine.state());

    let mut foo = MicroFoo::new();
    let mut bar = MicroBar::new();

    println!("{:?}", foo.confirm());
    println!("{:?}", foo.state());
    println!("{:?}", foo.reset());

    println!("{:?}", bar.ignore());
    println!("{:?}", bar.panic());
    println!("{:?}", bar.suplex());
    println!("{:?}", bar.state());

}
