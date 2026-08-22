// Cas nominal : action, machine à états, transitions, réception d'événement.
package StateMachine {
    item def SignalDemarrage;
    item def SignalArret;

    action def Demarrer;
    action def Arreter;

    state def EtatVehicule {
        entry action e1 : Demarrer;
        state arret;
        state roulant;
        transition t1 first arret accept SignalDemarrage then roulant;
        transition t2 first roulant accept SignalArret then arret;
    }
}
