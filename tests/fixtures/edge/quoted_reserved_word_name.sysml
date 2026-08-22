// Cas limite : un mot réservé SysML v2 utilisé comme nom, correctement échappé.
package QuotedReservedWordName {
    part def Robot {
        part 'part' : Robot;
        part 'attribute' : Robot;
    }
}
