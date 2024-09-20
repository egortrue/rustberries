// L1.20

///////////////////////////////////////////////////
// Старый проц

struct ProcessorX86 {}

impl ProcessorX86 {
    fn calculate(&self, data: i16) {
        println!("brahhhh, bah bah: {data}")
    }
}

///////////////////////////////////////////////////
// Новый проц

struct ProcessorX64 {}

trait ModernComputation {
    fn calculate(&self, data: i64);
}

impl ModernComputation for ProcessorX64 {
    fn calculate(&self, data: i64) {
        println!("Your data, sir: {data}");
    }
}

///////////////////////////////////////////////////
// Адаптер для старых процессоров

struct OldProcessorAdapter {
    old_processor: ProcessorX86,
}

impl ModernComputation for OldProcessorAdapter {
    fn calculate(&self, data: i64) {
        self.old_processor.calculate(data as i16);
    }
}

///////////////////////////////////////////////////

use std::{collections::LinkedList, i64};

fn main() {
    // Создаем парк из процессоров
    let mut processors: LinkedList<&dyn ModernComputation> = LinkedList::new();
    processors.push_back(&ProcessorX64 {});
    processors.push_back(&ProcessorX64 {});

    // Пытаемся добавить старый процессор
    let old_processor = ProcessorX86 {};
    // processors.append(&old_processor); <-- не работает, так как нужен адаптер

    // Адаптируем старый процессор
    let adapted_processor = OldProcessorAdapter {
        old_processor: old_processor,
    };
    processors.push_back(&adapted_processor);

    // Запускаем все наши процессоры
    for processor in processors {
        processor.calculate(i64::MAX);
    }

    /*

    Получаем ответ >_<

    Your data, sir: 9223372036854775807
    Your data, sir: 9223372036854775807
    brahhhh, bah bah: -1

     */
}
